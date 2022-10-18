#!/usr/bin/bash

# Run "normal" tests first with miri
# and only when sucessful, run the compile tests,
# which are marked as ignored (they don't work with miri).

cd own_ref_tests && \
cargo +nightly miri test && \
cargo test -- --ignored
