# Plugin Protocol
Communication with the client is done over TCP. Every interaction is sent as a JSON object. We refer to these objects as messages. Messages are always newline-separated and single-line. The rationale for JSON messages is that Python has a built-in JSON parser, so plugin developers can avoid the headache of custom parsing.

## Messages
Every message contains a `kind` field that denotes what kind of message it is. Any additional fields are then assigned depending on the kind.

```JSON
{ "kind": "message kind goes here" }
```

### Push
The push message is sent from either client or server and pushes object updates to the recepiant. It contains an array (`objects`) of each object, the format for which is provided later
in this document.

```JSON
{
  "kind": "push",
  "objects": []
}
```

## Objects
Objects represent logical parts of a executable image. Currently, there are 3 types of objects: Types, Functions, and Globals.

### Types
Types represent a format/layout of data that can be stored in memory
```JSON
{
  "kind": "type",
  "size": 4,
  "alignment": 4,
  "info": {}
}
```

Every type has an `info` field that contains additional information.
```JSON
{
  "kind": "type info kind goes here"
}
```

`kind` can be one of the following:
- `struct`
```JSON
{
  "kind": "struct",
  "fields": [
    {
      "name": "field name here",
      "type": {}, // TypeRef
      "offset": 0,
    }
  ]
}
```
- `enum`
```JSON
{
  "kind": "enum",
  "values": [
    {
      "name": "name goes here",
      "value": 1
    }
  ]
}
```
- `union`
```JSON
{
  "kind": "union",
  "fields": [
    {
      "name": "field name here",
      "type": {}, // TypeRef
    }
  ]
}
```
#### TypeRef
Sometimes objects need to refer to a type. This is done with special structure called a type reference. Here is the format for type references (`TypeRef`s), as well as a few reserved keywords.

There are three kinds of `TypeRef`s:
- `pointer` delineates a pointer type. The format is always `{ "kind": "pointer", "depth": 1, "name": "type name here" }`, where `depth` indicates the number of asterisks
that would normally be in the C type.
- `value` delineates a value type. The format for these is `{ "kind": "value", "name": "type name here" }`.
- Reserved types are special TypeRefs that refer to a builtin type. The format for these is `{ "kind": "reserved type name here" }`.

The reserve types are:
- `u8`, `u16`, `u32`, `u64` are all unsigned integer types of their respective bit sizes
- `i8`, `i16`, `i32`, `i64` are all signed integer types in similar fashion
- `f32`, `f64` are IEEE floating point types of the 32-bit and 64-bit variant

### Functions
Function represent code. Pretty simple.

```JSON
{
  "kind": "function",
  "name": "function name here",
  "arguments": [],
  "return_type": {} // TypeRef
}
```
