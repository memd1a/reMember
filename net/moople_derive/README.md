# Moople derive

** Write maple packets as rust structures **



## Examples

```rust

#[derive(MooplePacket)]
pub struct Packet {
    test8: u8,
    test16: u16,
    test32: u32,
    test64: u64,
}

fn check_name_even(name: &str) -> bool {
    name.len() % 2 == 0
}

#[derive(MooplePacket)]
pub struct PacketComplex<'a, T> {
    // Take a reference to avoid allocations
    name: &'a str,
    // If the length of name is even decode `a`
    #[pkt(if(field = "name", cond = "check_name_even"))]
    a: CondOption<u16>,
    // If is even then go for String elsewise for a bool
    #[pkt(either(field = "name", cond = "check_name_even"))]
    b: CondEither<String, bool>,
}
```

## Features

Supports:

* Lifetimes
* Generics
* All types which implement the *EncodePacket* + *DecodePacket* trait
* Conditional types `CondOption<T>` + `CondEither<L, R>`, with pkt(if(..)), pkt(either(..)) attribute
* size attribute pkt(size = "len field") for types which implement DecodePacketSized

## ToDo

* Split the derive into Encode and Decode