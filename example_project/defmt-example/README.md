# Run

You are now all set to `cargo-run` your first `defmt`-powered application!
There are some examples in the `src/bin` directory.

Start by `cargo run`-ning `defmt-example/src/bin/hello.rs`:

```console
$ # `rb` is an alias for `run --bin`
$ cargo rb hello
    Finished dev [optimized + debuginfo] target(s) in 0.03s
flashing program ..
DONE
resetting device
0.000000 INFO Hello, world!
(..)

$ echo $?
0
```

If you're running out of memory (`flip-link` bails with an overflow error), you can decrease the size of the device memory buffer by setting the `DEFMT_RTT_BUFFER_SIZE` environment variable. The default value is 1024 bytes, and powers of two should be used for optimal performance:

```console
$ DEFMT_RTT_BUFFER_SIZE=64 cargo rb hello
```

## Running tests

The template comes configured for running unit tests and integration tests on the target.

Unit tests reside in the library crate and can test private API; the initial set of unit tests are in `src/lib.rs`.
`cargo test --lib` will run those unit tests.

```console
$ cargo test --lib
(1/1) running `it_works`...
└─ app::unit_tests::__defmt_test_entry @ src/lib.rs:33
all tests passed!
└─ app::unit_tests::__defmt_test_entry @ src/lib.rs:28
```

Integration tests reside in the `tests` directory; the initial set of integration tests are in `tests/integration.rs`.
`cargo test --test integration` will run those integration tests.
Note that the argument of the `--test` flag must match the name of the test file in the `tests` directory.

```console
$ cargo test --test integration
(1/1) running `it_works`...
└─ integration::tests::__defmt_test_entry @ tests/integration.rs:13
all tests passed!
└─ integration::tests::__defmt_test_entry @ tests/integration.rs:8
```

Note that to add a new test file to the `tests` directory you also need to add a new `[[test]]` section to `Cargo.toml`.
