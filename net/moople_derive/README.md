# Moople derive

** Write maple packets as rust structures **

## Examples

```rust

#[derive(MaplePacket)]
pub struct Packet {
    test8: u8,
    test16: u16,
    test32: u32,
    test64: u64,
}
```

## Features

Supports:
* Lifetimes
* Generics
* All types which implement the *EncodePacket* + *DecodePacket* trait


## ToDo

* Conditional fields, as in:
```rust
#[derive(MaplePacket)]
pub struct ConditionalPacket {
    test8: u8,
    #[maple_packet(skip_if = "test8 > 10")]
    test16: u16,
}
```
* Split the derive into Encode and Decode