const hello_wasm = import("../wasm_out/hello_wasm");

hello_wasm.then(h => {
    let grid = new h.Grid();
    console.log(grid);
});
