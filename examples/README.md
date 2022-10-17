# Running the examples

1. Generate the bindings: `ligen/examples/example $ cargo build --features=bindings`
2. Running the C example:
   1. Generate build directory: `ligen/examples/example-c $ mkdir build`
   2. Generate project files: `ligen/examples/example-c/build $ cmake ..`
   3. Build: `ligen/examples/example-c/build $ cmake --build .`
   4. Run: `ligen/examples/example-c/build/Debug/example-c`