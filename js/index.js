const hello_wasm = import("../wasm_out/hello_wasm");

hello_wasm.then(hello_wasm => {
    hello_wasm.greet("World!");
});
