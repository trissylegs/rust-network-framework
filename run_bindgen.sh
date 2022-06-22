#!/usr/bin/env bash

ISYSROOT="$(xcrun --sdk macosx --show-sdk-path)"

bindgen --objc-extern-crate     \
        --generate-block        \
        --block-extern-crate    \
        bindgen_header.h        \
        --                      \
        -x objective-c          \
        -isysroot" ${ISYSROOT}" \
        > src/sys.rs
