# cenums

C-like enum in Rust

## Introduction

In Rust, C-like enum can be defined like

```rust
#[repr(u32)]
enum Hoge {
    A = 1,
    B = 2,
}
```

However, when working with C API, this is not enough in some aspects;

* In Rust implementation, the enum must take any one of the declared element. On the other hand, in C, `enum` is just a syntax sugar of an integer type with some constants, which means it is valid to take values not listed in the constants.
* Rust `enum` is rather direct-sum type. Thus, the conversion from raw type (like `u32` in the above code) to the enum is not convenient.

Of course, these problems would be solved if we just use primitive types instead, but that spoils the power of typing.

This crate defines a macro `cenums!`, which helps to declare *struct* with some features.

* Easy declaration of constants like [bitflags](https://github.com/bitflags/bitflags)
* Helpful `Debug` implementations
* Conversion between the based type
* Function to return whether the value is defined
* List of all defined constants

The idea of this crate is inspired to [bitflags](https://github.com/bitflags/bitflags). Some codes are also owed to bitflags, modified under [the MIT license](https://github.com/bitflags/bitflags/blob/master/LICENSE-MIT).

## LICENSE

MIT