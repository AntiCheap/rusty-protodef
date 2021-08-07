
## Purpose

> **:warning: Warning**: This documentation is for *minecraft-data protodef*, different from [protodefc](https://github.com/ProtoDef-io/protodefc) despite same name and purpose.

[ProtoDef](https://github.com/ProtoDef-io/ProtoDef) is a project to simplify data parsing and serialization, started as a contribution to PrismarineJS. Protodef allows to describe structured data of internet protocols or file formats in a concise form using JSON. This can be used by language specific implementations to obtain actual code to parse and serialize data. Unlike [ProtoBuf]() this project is meant to allow easier creation of custom types and more flexibility. The first use for this was converting Minecraft's raw packets into objects and the other way round. Given the size of the game's protocol and its many existing versions, writing code by hand wouldn't be feasible. 

#### Projects
* **[protodef-yaml](https://extremeheat.github.io/protodef-yaml/)** compiles a more readable format into minecraft data protodef.
* **[node-protodef](https://github.com/ProtoDef-io/node-protodef)** is the first JavaScript implementation (compiled and interpreted).
* **[minecraft-data](https://github.com/PrismarineJS/minecraft-data)** uses protodef to speficy the game's protocol in each version.

## Natives

#### Numeric:
* **f32, f64**: floating points. ([IEEE 754](https://en.wikipedia.org/wiki/IEEE_754))
* **i8, i16, i32, i64**: two's complement integers.
* **u8, u16, u32, u64**: unsigned integers.
* **varint**: base 128 int32. ([Protocol Buffers](https://developers.google.com/protocol-buffers/docs/encoding#varints))

#### Primitives:
* **bool**: boolean value, zero or one byte.
* * fails when insufficient bytes.
* * fails when not 0x00 or 0x01.
* **cstring**: null terminated utf-8 string.
* * fails when 0x00 is not found.
* **void**: only usable inside switch.
* * never fails.
#### Countables:
* **array**: repetition of another type.
* **buffer**: chunk of binary data.
* **pstring**: utf-8 string of some length.
#### Structures:
* **bitfield**: groups numbers coming from bits.
* **container**: organizes other types inside it.
* **switch**: changes type with a weak comparison.
#### Utility:
* **count**: gets a countable when serializing.
* **mapper**: when parsing it first parses the type specified as 'type'
* **option**: can hold or not another type.

## Options
| Type   | Options                                                                                             | Parse                                                                                                                                                                                                                                             | Serialize                                                                                                                                                                                                                  |
|--------|-----------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| switch | { ?compareTo: Reference, ?compareToValue: String, fields: { [String]: Type, ... }, ?default: Type } | A comparison decides what data will be produced. The Type to parse for is                                                                                                                                                                         |                                                                                                                                                                                                                            |
| mapper | { type: Type, mappings: { [String]: String, ... }}                                                  | This process returns a string based upon a simple lookup. The parsed value of 'type' is compared with the keys of the 'mappings' object. If a matching key is found its value pair will be the output string. If no match is found parsing fails. | The mapping is done in the opposite way. If data is a string under a key of 'mappings', 'type' is serialized with such key as value, otherwise serializing fails. Multiple keys sharing the same value can be a problem.   |
| count  | { type: Type, countFor: Reference }                                                                 | Identical to parsing 'type'.                                                                                                                                                                                                                      | Instead of serializing data, 'type' is serialized with the length of the array/string/chunk referenced in 'countFor' as its value. If it's impossible to find the length or represent it with this type serializing fails. |
| option | Type                                                                                                | Parses a boolean, if its value is true it returns the parsed type found in options.                                                                                                                                                               | If no data to serialize is provided it serializes only a false boolean, otherwise it serializes a true boolean followed by the data.                                                                                       |
|        |                                                                                                     |                                                                                                                                                                                                                                                   |                                                                                                                                                                                                                            |



### Terms
* Counter is a string. If it pases as integer it represents a fixed length otherwise it's a Reference.
* Reference is a string. It is the relative path from the current type to another field.
* Definition is a string. It is a type definition, either string `"name"` or array `[name, options]`.

### List
! Please note: all comparisons are done converting numbers to string.

* switch: ({ ?compareTo: Reference, ?compareToValue: String, fields: { [String]: Definition, ... }, ?default: Definition })
* option: (Definition)
* array: ({ type: Definition, ?countType: Definition, ?count: Counter })
* container: ([ { name: String, type: Definition }, ... ])
* count: ({ type: Definition, countFor: Reference })
* buffer: ({ countType: Definition, ?count: Counter, ?rest: Boolean })
* bitfield: ([ { name: String, size: Number, ?signed: Boolean } ])
* mapper: ({ type: Definition, mappings: { String: String, ... } })
* pstring: ({ countType: Definition, ?count: Counter })
