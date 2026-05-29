// pkg/db_bridge.js
// JS implementations of the mq_db_* FFI functions called from Rust/WASM.

let _resultBuffer = null; // Holds the UTF-8 encoded response bytes

export function mq_db_query(ptr, len, url_ptr, url_len) {
    // We can't read WASM memory here yet (no wasm ref), so we
    // defer reading until the WASM instance is available.
    // Instead, we schedule the fetch using a global WASM ref set by the loader.
    _resultBuffer = null;

    const wasm = window.__wasm_instance;
    if (!wasm) {
        console.error("mq_db_query: No WASM instance found on window.__wasm_instance");
        return;
    }

    const memory = wasm.exports.memory;
    const bodyBytes  = new Uint8Array(memory.buffer, ptr, len);
    const urlBytes   = new Uint8Array(memory.buffer, url_ptr, url_len);
    const bodyString = new TextDecoder().decode(bodyBytes);
    const urlString  = new TextDecoder().decode(urlBytes);

    // Fire the fetch; store result in _resultBuffer when done
    fetch(urlString, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: bodyString,
    })
    .then(r => r.arrayBuffer())
    .then(buf => {
        _resultBuffer = new Uint8Array(buf);
    })
    .catch(err => {
        console.error("mq_db_query fetch error:", err);
        // Store an error JSON so Rust gets something back instead of timing out
        _resultBuffer = new TextEncoder().encode(JSON.stringify({ error: err.toString() }));
    });
}

export function mq_db_query_result_len() {
    return _resultBuffer ? _resultBuffer.length : 0;
}

export function mq_db_query_fill_result(ptr) {
    if (!_resultBuffer) return;
    const wasm = window.__wasm_instance;
    if (!wasm) return;
    const memory = wasm.exports.memory;
    const dest = new Uint8Array(memory.buffer, ptr, _resultBuffer.length);
    dest.set(_resultBuffer);
}

export function mq_db_query_clear_result() {
    _resultBuffer = null;
}