### Compiler.addTypes(types)

Add `types` which is an object with keys the name of the types and values the type definitions.

### Compiler.addProtocol(protocol, path)

Add types in `protocol` recursively. The protocol object is an object with keys `types` and namespace keys.
* The value of the `types` key is an object of type name to type definition.
* The value of the namespace key is a protocol object.

The `path` is an array of namespace keys which select a path of namespaces to be added to the protodef object.

### Compiler.compileSync()

Returns rust code of the compiled protocol.

```js
const fs = require('fs');
const protodef = require('protodef-rs');

const compiler = new protodef.Compiler();
compiler.addProtocol(require('./protocol.json'));
const output = compiler.compileSync();

fs.writeFileSync("rustcode.rs", output);
```
