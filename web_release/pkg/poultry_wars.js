let wasm;

export function set_wasm(val) {
    wasm = val;
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_externrefs;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
    };
        imports['env'] = {
        mq_db_query: (ptr, len, url_ptr, url_len) => window.__db_bridge.mq_db_query(ptr, len, url_ptr, url_len),
        mq_db_query_result_len: () => window.__db_bridge.mq_db_query_result_len(),
        mq_db_query_fill_result: (ptr) => window.__db_bridge.mq_db_query_fill_result(ptr),
        mq_db_query_clear_result: () => window.__db_bridge.mq_db_query_clear_result(),
    };
    return imports;
}

function initSync(module) {
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
}

export { initSync };
export default __wbg_init;
