# `rustflags` is not passed when using artifact dependency?

I asked question [here]().

This is a reproduction code for above question.

You can reproduce my problem in below steps.

1. run `cargo build` in root workspace directory.
2. run `cargo build` in `binary_deps` directory.
3. compare ELF type between `target/debug/binary_deps` and `target/debug/deps/artifact/binary-deps-<id>/bin/binary_deps-<id>`