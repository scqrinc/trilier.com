
import * as wasm from "app-wasm";

wasm.dom_content_loaded();
document.querySelector('#app main form').addEventListener('submit', wasm.form_submit);
