use crate::{StateRef, utils::build_headers, utils::fetch_json};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{RequestCredentials, RequestInit, RequestMode};

#[wasm_bindgen]
pub struct Post {
    pub(crate) state: StateRef,
}

#[wasm_bindgen]
impl Post {
    #[wasm_bindgen]
    pub fn create(&self, title: String, content: String) -> Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let s = state.borrow();
            let url = format!("{}{}", s.base_url, "/post");
            let body = serde_json::json!({"title": title, "content": content}).to_string();
            let opts = RequestInit::new();
            opts.set_method("POST");
            opts.set_mode(RequestMode::Cors);
            opts.set_credentials(RequestCredentials::Include);
            opts.set_body(&JsValue::from_str(&body));
            let req = web_sys::Request::new_with_str_and_init(&url, &opts).map_err(|e| e)?;
            build_headers(&s, true, true, &req).map_err(|e| e)?;
            let res = fetch_json(&req).await?;
            Ok(res)
        })
    }

    #[wasm_bindgen]
    pub fn update(&self, id: String, title: String, content: String) -> Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let s = state.borrow();
            let url = format!("{}{}{}", s.base_url, "/post/", id);
            let body = serde_json::json!({"title": title, "content": content}).to_string();
            let opts = RequestInit::new();
            opts.set_method("PATCH");
            opts.set_mode(RequestMode::Cors);
            opts.set_credentials(RequestCredentials::Include);
            opts.set_body(&JsValue::from_str(&body));
            let req = web_sys::Request::new_with_str_and_init(&url, &opts).map_err(|e| e)?;
            build_headers(&s, true, true, &req).map_err(|e| e)?;
            let res = fetch_json(&req).await?;
            Ok(res)
        })
    }

    #[wasm_bindgen]
    pub fn delete(&self, id: String) -> Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let s = state.borrow();
            let url = format!("{}{}{}", s.base_url, "/post/", id);
            let opts = RequestInit::new();
            opts.set_method("DELETE");
            opts.set_mode(RequestMode::Cors);
            opts.set_credentials(RequestCredentials::Include);
            let req = web_sys::Request::new_with_str_and_init(&url, &opts).map_err(|e| e)?;
            build_headers(&s, true, true, &req).map_err(|e| e)?;
            let res = fetch_json(&req).await?;
            Ok(res)
        })
    }

    #[wasm_bindgen]
    pub fn get_by_id(&self, id: String) -> Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let s = state.borrow();
            let url = format!("{}{}{}", s.base_url, "/post/", id);
            let opts = RequestInit::new();
            opts.set_method("GET");
            opts.set_mode(RequestMode::Cors);
            opts.set_credentials(RequestCredentials::Include);
            let req = web_sys::Request::new_with_str_and_init(&url, &opts).map_err(|e| e)?;
            let res = fetch_json(&req).await?;
            Ok(res)
        })
    }

    #[wasm_bindgen]
    pub fn gets_by_author(&self, email: String) -> Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let s = state.borrow();
            let url = format!("{}{}{}", s.base_url, "/post/author/", email);
            let opts = RequestInit::new();
            opts.set_method("GET");
            opts.set_mode(RequestMode::Cors);
            opts.set_credentials(RequestCredentials::Include);
            let req = web_sys::Request::new_with_str_and_init(&url, &opts).map_err(|e| e)?;
            let res = fetch_json(&req).await?;
            Ok(res)
        })
    }

    #[wasm_bindgen]
    pub fn gets_me(&self) -> Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let s = state.borrow();
            let url = format!("{}{}", s.base_url, "/post/me");
            let opts = RequestInit::new();
            opts.set_method("GET");
            opts.set_mode(RequestMode::Cors);
            opts.set_credentials(RequestCredentials::Include);
            let req = web_sys::Request::new_with_str_and_init(&url, &opts).map_err(|e| e)?;
            build_headers(&s, true, false, &req).map_err(|e| e)?;
            let res = fetch_json(&req).await?;
            Ok(res)
        })
    }
}
