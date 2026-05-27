const fs = require('fs');

// Path to your generated JS file. Change the default or pass it as a CLI argument.
const filePath = process.argv[2] || 'pkg/poultry_wars.js';

if (!fs.existsSync(filePath)) {
    console.error(`File not found: ${filePath}`);
    process.exit(1);
}

let code = fs.readFileSync(filePath, 'utf8');

// 1. Remove the 'env' import (Miniquad/Macroquad provides this)
code = code.replace(/import \*\s+as\s+__wbg_star0\s+from\s+'env';\n*/, '');

// 2. Export `set_wasm` so Miniquad can inject the wasm instance
if (!code.includes('export function set_wasm')) {
    code = code.replace(/(let wasm;\n)/, '$1\nexport function set_wasm(val) {\n    wasm = val;\n}\n');
}

// 3. Remove `EXPECTED_RESPONSE_TYPES` and `__wbg_load` blocks entirely
// (Matches from EXPECTED_RESPONSE_TYPES down to the start of __wbg_get_imports)
code = code.replace(/const EXPECTED_RESPONSE_TYPES[\s\S]*?(?=function __wbg_get_imports)/, '');

// 4. Modify __wbg_get_imports
// Remove the env assignment
code = code.replace(/imports\['env'\]\s*=\s*__wbg_star0;\n*/, '');
// Change the return statement to return the flat wbg object
code = code.replace(/return imports;\n\}(?=\n*function __wbg_finalize_init)/, 'return imports.wbg;\n}');

// 5. Replace __wbg_finalize_init, initSync, and __wbg_init with the stripped-down versions
const bottomBoilerplate = `function initSync(module) {
    if (wasm !== undefined) return wasm;
    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return wasm;
}

async function __wbg_init(module_or_path) {
    const imports = __wbg_get_imports();
    return imports;
}\n\n`;

code = code.replace(/function __wbg_finalize_init[\s\S]*?(?=export \{ initSync \})/, bottomBoilerplate);

fs.writeFileSync(filePath, code);
console.log(`✅ Successfully patched ${filePath} for Macroquad/Miniquad!`);