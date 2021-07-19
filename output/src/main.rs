mod protodef;

use protodef::types;
use protodef::Protodef;

fn main() {
    //having 16 bytes or less would be a dream.
    println!("{}", std::mem::size_of::<Protodef>());

    //The number before 13 is
    let vec = vec![
        65, 65, 0, 0, 1, 2, 3, 4, 5, 6, 7, 255, 255, 255, 1, 13, 14, 15, 255, 10, 11, 0, 255, 0, 0,
    ];

    let mut cursor = &vec[..];
    let data = parse(&mut cursor);
    println!("parse: {:?}", data);

    let mut out = vec![];
    serial(&data.unwrap(), &mut out);
    println!("serial: {:?}", out);

    println!("reparse: {:?}", parse(&mut &out[..]));
}

//The following code was obtained from "node index.js":

pub fn parse(input: &mut &[u8]) -> Option<Protodef> {
    Some(scope!(root_0, Protodef::new_object(), {
        root_0.set("container", Protodef::new_object());
        let here_1 = root_0.get_mut("container")?;
        here_1.set("sub", types::cstring::parse(input)?);
        here_1.set("zero", types::f64::parse(input)?);
        here_1.set("alfa", Protodef::new_object());
        let here_2 = here_1.get_mut("alfa")?;
        here_2.set("beta", types::u8::parse(input)?);
        //end here_2
        here_1.set("alfetta", Protodef::new_object());
        let here_2 = here_1.get_mut("alfetta")?;
        here_2.set("beta", types::u8::parse(input)?);
        //end here_2
        here_1.set("gamma", types::u8::parse(input)?);
        here_1.set(
            "delta",
            Protodef::String({
                match types::u8::real_parse(input)? {
                    0 => Some("zero"),
                    1 => Some("one"),
                    _ => None,
                }?
                .to_string()
            }),
        );
        //end here_1
        root_0.set("omega", types::i8::parse(input)?);
    }))
}

pub fn serial(root_0: &Protodef, output: &mut Vec<u8>) -> Option<()> {
    let here_1 = root_0.get("container")?;
    types::cstring::serial(here_1.get("sub")?, output)?;
    types::f64::usize_serial(
        {
            //Count for: sub
            root_0.get("container")?.get("sub")?.to_length()?
        },
        output,
    )?;
    let here_2 = here_1.get("alfa")?;
    types::u8::serial(here_2.get("beta")?, output)?;
    //end here_2
    let here_2 = here_1.get("alfetta")?;
    types::u8::serial(here_2.get("beta")?, output)?;
    //end here_2
    types::u8::serial(here_1.get("gamma")?, output)?;
    types::u8::real_serial(
        &{
            match here_1.get("delta")?.as_str()? {
                "zero" => Some(0),
                "one" => Some(1),
                _ => None,
            }?
        },
        output,
    );
    //end here_1
    types::i8::serial(root_0.get("omega")?, output)?;
    Some(())
}
