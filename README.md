# mlm-unstable

The goal is to learn rust, rust ffi and so via writing a wrapper on top of
malamute, which is typical ZeroMQ CLASS based project.

There are **no** guarantees about stability, or about provided sane interface for malamute.

Code is under MIT license

## How to build

1. Build a malamute https://github.com/zeromq/malamute/#building-malamute
2. use cargo test to build the tests

Tested on rustc-1.9.

## How to contribute

Just send a PR on github

## Next problem
Provide safe and idiomatic interface on top of extern unsafe functions.
