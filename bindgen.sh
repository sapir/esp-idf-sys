#!/usr/bin/env bash

set -e

. ./setenv.sh

COMPS=$IDF_PATH/components
# `xtensa-esp32-elf-gcc --print-sysroot` outputs nothing
gcc_fullpath=$(which xtensa-esp32-elf-gcc)
: "${SYSROOT:=${gcc_fullpath%/*}/../xtensa-esp32-elf}"
TARGET=xtensa-esp32-none-elf

: "${BINDGEN:=bindgen}"
: "${LIBCLANG_PATH:=../llvm-project/llvm/build/lib}"
CLANG_FLAGS="\
    --sysroot=$SYSROOT \
    -Ibuild/include/ \
    -D__bindgen \
    --target=$TARGET \
    -x c"

for INC in $(ls -d "$COMPS"/**/*/include); do
    CLANG_FLAGS="${CLANG_FLAGS} -I$INC"
done
for INC in $(ls -d "$COMPS"/*/include); do
    CLANG_FLAGS="${CLANG_FLAGS} -I$INC"
done
CLANG_FLAGS="${CLANG_FLAGS} \
    -I$COMPS/esp_websocket_client/include \
    -I$COMPS/lwip/include/apps \
    -I$COMPS/lwip/include/apps/sntp \
    -I$COMPS/lwip/lwip/src/include \
    -I$COMPS/lwip/port/esp32/include \
    -I$COMPS/newlib/platform_include"

generate_bindings()
{
    readonly crate="$1"

    cd "$crate"

    # --no-rustfmt-bindings because we run rustfmt separately with regular rust
    LIBCLANG_PATH="$LIBCLANG_PATH" \
    "$BINDGEN" \
        --use-core \
        --no-layout-tests \
        --no-rustfmt-bindings \
        $BINDGEN_FLAGS \
        --output esp-idf-sys/src/bindings.rs \
        esp-idf-sys/src/bindings.h \
        -- $CLANG_FLAGS

    rustup run stable rustfmt esp-idf-sys/src/bindings.rs
}

generate_bindings "$@"
