use crate::{
    StateRef,
    utils::{fetch_bytes, fetch_text},
};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{RequestCredentials, RequestInit, RequestMode};

#[wasm_bindgen]
pub struct General {
    pub(crate) state: StateRef,
}

#[wasm_bindgen]
impl General {
    #[wasm_bindgen]
    pub fn health(&self) -> Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let s = state.borrow();
            let url = format!("{}{}", s.base_url, "/health");
            let opts = RequestInit::new();
            opts.set_method("GET");
            opts.set_mode(RequestMode::Cors);
            opts.set_credentials(RequestCredentials::Include);
            let req = web_sys::Request::new_with_str_and_init(&url, &opts).map_err(|e| e)?;
            let txt = fetch_text(&req).await?;
            Ok(JsValue::from_str(&txt))
        })
    }

    #[wasm_bindgen]
    pub fn ping(&self) -> Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let s = state.borrow();
            let url = format!("{}{}", s.base_url, "/ping");
            let opts = RequestInit::new();
            opts.set_method("GET");
            opts.set_mode(RequestMode::Cors);
            opts.set_credentials(RequestCredentials::Include);
            let req = web_sys::Request::new_with_str_and_init(&url, &opts).map_err(|e| e)?;
            let txt = fetch_text(&req).await?;
            Ok(JsValue::from_str(&txt))
        })
    }

    #[wasm_bindgen]
    pub fn media(&self, filename: String) -> Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let s = state.borrow();
            let url = format!("{}{}", s.base_url, format!("/media/{}", filename));
            let opts = RequestInit::new();
            opts.set_method("GET");
            opts.set_mode(RequestMode::Cors);
            opts.set_credentials(RequestCredentials::Include);
            let req = web_sys::Request::new_with_str_and_init(&url, &opts).map_err(|e| e)?;
            let bytes = fetch_bytes(&req).await?;
            Ok(bytes.into())
        })
    }
}
