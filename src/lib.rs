use js_sys::Promise;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::console::log_1;

#[wasm_bindgen]
extern "C" {
    fn sleep(ms: f64) -> Promise;
}

#[wasm_bindgen]
pub fn console_log(s: &str) {
    #[allow(unused_unsafe)]
    unsafe {
        log_1(&JsValue::from(String::from(s)));
    }
}

#[wasm_bindgen]
pub async fn timer(count: u32) -> Result<JsValue, JsValue> {
    for i in 1..count {
        unsafe {
            JsFuture::from(sleep(1000f64)).await?;
        }
        console_log(&i.to_string());
    }

    Ok(JsValue::undefined())
}
