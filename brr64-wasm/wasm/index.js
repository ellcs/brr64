
const runWasm = async () => {
  // Instantiate our wasm module
  const rustWasm = await wasmInit("./brr64_bg.wasm");

  // Call our exported function
  const helloString = add_wasm_by_example_to_string("Hello from ");

  // Log the result to the console
  domConsoleLog(helloString);
};
runWasm();
