use crate::State;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::Request;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);
}

pub(crate) async fn get_cookie(name: &str) -> Option<String> {
    let window = web_sys::window()?;
    let store = window.cookie_store();
    let future = wasm_bindgen_futures::JsFuture::from(store.get_with_name(name));
    let result = future.await.ok()?;
    if let Ok(value) = js_sys::Reflect::get(&result, &JsValue::from_str("value")) {
        return value.as_string();
    }

    None
}

pub(crate) fn build_headers(
    state: &State,
    auth: bool,
    csrf: bool,
    req: &Request,
) -> Result<(), JsValue> {
    req.headers()
        .set("Accept", "application/json")
        .map_err(|e| e)?;
    req.headers()
        .set("Content-Type", "application/json")
        .map_err(|e| e)?;
    if auth {
        if let Some(token) = &state.jwt {
            req.headers()
                .set("Authorization", &format!("Bearer {}", token))
                .map_err(|e| e)?;
        }
    }
    if csrf {
        req.headers()
            .set("x-csrf-token", &state.csrf)
            .map_err(|e| e)?;
    }
    Ok(())
}

pub(crate) async fn fetch_text(req: &Request) -> Result<String, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(req)).await?;
    let resp: web_sys::Response = resp_value
        .dyn_into()
        .map_err(|_| JsValue::from_str("not response"))?;
    let text = wasm_bindgen_futures::JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap_or_default())
}

pub(crate) async fn fetch_json(req: &Request) -> Result<JsValue, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(req)).await?;
    let resp: web_sys::Response = resp_value
        .dyn_into()
        .map_err(|_| JsValue::from_str("not response"))?;
    let json = wasm_bindgen_futures::JsFuture::from(resp.json()?).await?;
    Ok(json)
}

pub(crate) async fn fetch_bytes(req: &Request) -> Result<js_sys::Uint8Array, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(req)).await?;
    let resp: web_sys::Response = resp_value
        .dyn_into()
        .map_err(|_| JsValue::from_str("not response"))?;
    let ab = wasm_bindgen_futures::JsFuture::from(resp.array_buffer()?).await?;
    let u8 = js_sys::Uint8Array::new(&ab);
    Ok(u8)
}
