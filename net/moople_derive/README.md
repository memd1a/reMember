# Moople derive

** Write maple packets as rust structures **

TODO:
rename skip_if to not_skip_if or sth not sure yet

## Examples

```rust

#[derive(MaplePacket)]
pub struct Packet {
    test8: u8,
    test16: u16,
    test32: u32,
    test64: u64,
}

fn check_name_even(name: &str) -> bool {
    name.len() % 2 == 0
}

#[derive(MaplePacket)]
pub struct PacketComplex<'a, T> {
    // Lifetime supported to avoid allocations
    name: &'a str,
    // If the length of name is even decode `a`
    #[maple_packet(skip_if(field = "name", cond = "check_name_even"))]
    a: CondOption<u16>,
    // If is even then go for String elsewise for a bool
    #[maple_packet(either(field = "name", cond = "check_name_even"))]
    b: CondEither<String, bool>,
}
```

## Features

Supports:
* Lifetimes
* Generics
* All types which implement the *EncodePacket* + *DecodePacket* trait
* Conditional types `CondOption<T>` + `CondEither<L, R>`


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