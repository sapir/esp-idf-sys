#!/usr/bin/env bash

set -e

. ./setenv.sh

COMPS=$IDF_PATH/components
: "${SYSROOT:=$(xtensa-esp32-elf-gcc --print-sysroot)}"
TARGET=xtensa-esp32-none-elf

: "${BINDGEN:=bindgen}"
: "${LIBCLANG_PATH:=../llvm-project/llvm/build/lib}"
CLANG_FLAGS="\
    --sysroot=$SYSROOT \
    -I"$(pwd)" \
    -D__bindgen \
    --target=$TARGET \
    -x c"

for comp in $COMPS/*; do
    dirs=$((cat $comp/component.mk && echo '$(info ${COMPONENT_ADD_INCLUDEDIRS})') | make -f - 2>/dev/null; true)
    for dir in $dirs; do
        CLANG_FLAGS="${CLANG_FLAGS} -I$comp/$dir"
    done
done
for INC in $(ls -d "$COMPS"/**/*/include); do
    CLANG_FLAGS="${CLANG_FLAGS} -I$INC"
done
for INC in $(ls -d "$COMPS"/*/include); do
    CLANG_FLAGS="${CLANG_FLAGS} -I$INC"
done

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
