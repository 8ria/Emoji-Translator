use wasm_bindgen::prelude::*;
use subslay::EmojiStylist;
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
pub struct WasmStylist {
    inner: EmojiStylist,
}

#[wasm_bindgen]
impl WasmStylist {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WasmStylist, JsValue> {
        EmojiStylist::new()
            .map(|inner| WasmStylist { inner })
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen]
    pub fn slay(&self, input: &str) -> JsValue {
        to_value(&self.inner.slay(input)).unwrap()
    }
}
