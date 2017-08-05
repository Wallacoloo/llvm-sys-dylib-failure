When using the `llvm-sys >= 39.0` crate inside a dylib, the dylib fails to locate all symbols at startup.

To reproduce:
```
$ cargo build
$ cd target/debug
$ ./load
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Custom(Custom { kind: Other, error: StringError("./libllvm_so.so: undefined symbol: history") }) }', /checkout/src/libcore/result.rs:860:4
```

Note that this problem does *not* occur with llvm-sys 38.0.5 (even when the OS llvm is 4.0.1).

This reproduces on both rustc 1.19.0 stable and nightly.
OS is Arch Linux with llvm 4.0.1
