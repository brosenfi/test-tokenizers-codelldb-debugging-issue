#!/bin/bash

cd /opt/llvm-project
git clone --branch "llvmorg-21.1.3" --depth 1 https://github.com/llvm/llvm-project.git .

mkdir build
cd build
cmake -G Ninja \
    -DLLVM_ENABLE_PROJECTS="clang;lldb" \
    -DLLDB_EXPORT_ALL_SYMBOLS=1 \
    -DCMAKE_BUILD_TYPE=Debug \
    -DLLVM_USE_LINKER=lld \
    -DLLVM_RAM_PER_LINK_JOB=12000 \
    ../llvm
ninja lldb lldb-server
