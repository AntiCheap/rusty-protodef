"use strict";

const methods = {
    here() {
        const lows = this.under.length;
        return lows === 0 ? this.root : `here_${lows}`;
    },
    setHere(code, gets) {
        if (this.isScope) return code;
        //return `let ${this.here()} = ${this.getter(gets)};\n${code}`;
        return `let ${this.here()} = ${this.getter(gets)};\n${code}\n//end ${this.here()}`;
    },
    getter(gets = "get") {
        //Requires rust .get .get_mut and .set Protodef methoods.
        if (this.isScope) return `${this.root}.as_ref()`;
        return `${this.upper().here()}.${gets}("${this.last}")?`;
    },
    setter(x, post = "") {
        if (this.isScope) {
            // The rust macro initializes the variable as Void(),
            // this way referencing it won't give compile error.
            return `scope!(${this.root}, ${x}, ${block(post)})`;
        } else {
            //This means you are inside a container or bitfield.
            if (post.length) post = `\n${this.setHere(post, "get_mut")}`;
            return `${this.getter("set").slice(0, -2)}, ${x});${post}`;
        }
    },
    relative(aim) {
        if (!aim || typeof aim !== "string") throw Error();
        const path = this.move(["..", ...aim.split("/")]);

        if (path.isScope) return `${path.root}.as_ref()`;
        const gets = path.under.map((x) => `.get("${x}")?`);
        return [path.root, ...gets].join("");
    },
};

function simpleType(name) {
    return (path) => ({
        parse: path.setter(`types::${name}::parse(input)?`),
        serial: `types::${name}::serial(${path.getter()}, output)?;`
    });
}

function simpleNumber(name) {
    const simple = simpleType(name);
    return (path) => ({
        ...simple(path), number: true,
        usizeSerial: (x) => `types::${name}::usize_serial(${x}, output)?;`,
        realSerial: (x) => `types::${name}::real_serial(${x}, output);`,
        usizeParse: `types::${name}::usize_parse(input)?`,
        realParse: `types::${name}::real_parse(input)?`,
    });
}

const numbers = {
    // Unsigned integers.
    u8: simpleNumber('u8'),
    u16: simpleNumber('u16'),
    u32: simpleNumber('u32'),
    u64: simpleNumber('u64'),
    // Signed integers.
    i8: simpleNumber('i8'),
    i16: simpleNumber('i16'),
    i32: simpleNumber('i32'),
    i64: simpleNumber('i64'),
    // Floating points.
    f32: simpleNumber('f32'),
    f64: simpleNumber('f64'),
    // Variable sized.
    varint: simpleNumber('varint')
};

const primitives = {
    bool: simpleType('bool'),
    cstring: simpleType('cstring'),
    //Void is accepted only in switch.
};

const utility = {
    mapper(path, opts) {
        const { type, mappings } = opts;
        if (!type || !mappings) throw Error();

        //Location of compiling doesn't matter.
        const flag = path.scoped().compile(type);
        if (!flag.number) throw Error();

        if (typeof mappings !== "object") throw Error();
        const entries = Object.entries(mappings)
            .map(([key, val]) => [Number(key), val]);

        entries.forEach(([num, val]) => {
            if (typeof val !== "string") throw Error();
            if (isNaN(num)) throw Error();
        });

        const maps = entries.map(([num, val]) => {
            const parse = `${num} => Some("${val}"),`;
            const serial = `"${val}" => Some(${num}),`;
            return { parse, serial };
        }).concat({
            serial: "_ => None,",
            parse: "_ => None,",
        });

        const encode = block(maps.map((x) => x.parse).join("\n"));
        const decode = block(maps.map((x) => x.serial).join("\n"));

        const parsing = `match ${flag.realParse} ${encode}?.to_string()`;
        const parse = path.setter(`Protodef::String(${block(parsing)})`);

        const serialing = `match ${path.getter()}.as_str()? ${decode}?`;
        const serial = flag.realSerial(`&${block(serialing)}`);

        return { serial: serial, parse: parse };
    },
    count(path, opts) {
        const { type, countFor } = opts;
        const kind = path.compile(type);
        if (!kind.number) throw Error();

        const target = path.relative(countFor);
        const length = `//Count for: ${countFor}\n${target}.to_length()?`;
        const serial = kind.usizeSerial(block(length));

        return { parse: kind.parse, serial };
    }
};

const types = {
    ...numbers,
    ...primitives,
    ...utility,
    container(path, opts) {
        //todo: anonymous bitfield and switch.
        const holds = opts.flatMap((x) => {
            const { type, name, anon } = x;
            if (Boolean(anon)) {
                const code = path.compile(type);
                if (!code.anon) throw Error();
                return code.anon();
            } else {
                if (!check(name)) throw Error();
                const here = path.into(name);
                return [here.compile(type)];
            }
        });

        const serialing = holds.map((x) => x.serial).join("\n");
        const parsing = holds.map((x) => x.parse).join("\n");

        const parse = path.setter("Protodef::new_object()", parsing);
        const serial = path.setHere(serialing);

        return { parse, serial, anon: () => holds };
    },
};

module.exports = { methods, types, finalize: typeFinalizer };

function tab(input, count = 4) {
    return input.split('\n')
        .map((x) => `${" ".repeat(count)}${x}`)
        .join('\n');
}

function block(x) {
    const text = x.trim();
    if (text.length === 0) return '{}';
    return `{\n${tab(text)}\n}`;
}

function check(x) {
    if (!x) return false;
    return typeof x === "string";
}

function typeFinalizer(code) {
    const parseCode = block(`Some(${code.parse})`);
    const serialCode = block(`${code.serial}\nSome(())`.trim());

    const parse = `pub fn parse(input: &mut &[u8]) -> Option<Protodef> ${parseCode}`;
    const serial = `pub fn serial(root_0: &Protodef, output: &mut Vec<u8>) -> Option<()> ${serialCode}`;

    return { parse, serial };
}