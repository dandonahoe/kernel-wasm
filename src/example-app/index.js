// index.js
import './my_wasm_function.wasm';


console.log('Hello, everything is working!');

// Load WebAssembly and call a function instead of using inline machine code.
(async () => {

    const response = await fetch('module.wasm');
    const buffer = await response.arrayBuffer();
    const wasmModule = await WebAssembly.instantiate(buffer);
    
    // Calling WebAssembly function exported from Wasm module
    const value = wasmModule.instance.exports.my_wasm_function(10, 20);

    console.log('Direct Wasm woo! Result from Wasm:', value);

})();
