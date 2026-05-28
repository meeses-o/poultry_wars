const fs = require('fs');

const filePath = process.argv[2] || 'pkg/poultry_wars.js';

if (!fs.existsSync(filePath)) {
    console.error(`File not found: ${filePath}`);
    process.exit(1);
}

let code = fs.readFileSync(filePath, 'utf8');

// 1. Remove the bare 'env' import — Miniquad + our bridge handles this
code = code.replace(/import \*\s+as\s+__wbg_star0\s+from\s+'env';\n*/, '');

// 2. Export set_wasm so Miniquad can inject the wasm instance
if (!code.includes('export function set_wasm')) {
    code = code.replace(/(let wasm;\n)/, '$1\nexport function set_wasm(val) {\n    wasm = val;\n}\n');
}

// 3. Remove EXPECTED_RESPONSE_TYPES and __wbg_load blocks
code = code.replace(/const EXPECTED_RESPONSE_TYPES[\s\S]*?(?=function __wbg_get_imports)/, '');

// 4. In __wbg_get_imports:
//    - Remove the line that assigned __wbg_star0 (already deleted above, belt+suspenders)
//    - BUT: do NOT delete imports['env'] — instead inject our bridge functions
code = code.replace(/imports\['env'\]\s*=\s*__wbg_star0;\n*/, '');

// Inject our bridge into imports['env'] right before the return statement
// so the WASM binary can find mq_db_query etc.
code = code.replace(
    /return imports;\n\}(?=\n*function __wbg_finalize_init)/,
    `imports['env'] = {
        mq_db_query: (ptr, len, url_ptr, url_len) => window.__db_bridge.mq_db_query(ptr, len, url_ptr, url_len),
        mq_db_query_result_len: () => window.__db_bridge.mq_db_query_result_len(),
        mq_db_query_fill_result: (ptr) => window.__db_bridge.mq_db_query_fill_result(ptr),
        mq_db_query_clear_result: () => window.__db_bridge.mq_db_query_clear_result(),
    };
    return imports;\n}`,
);

// 5. Replace the init boilerplate
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
    return imports.wbg;
}\n\n`;

code = code.replace(/function __wbg_finalize_init[\s\S]*?(?=export \{ initSync \})/, bottomBoilerplate);

fs.writeFileSync(filePath, code);
console.log(`Successfully patched ${filePath}`);