use crate::{
    StateRef,
    utils::{build_headers, fetch_json},
};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{RequestCredentials, RequestInit, RequestMode};

#[wasm_bindgen]
pub struct User {
    pub(crate) state: StateRef,
}

#[wasm_bindgen]
impl User {
    #[wasm_bindgen]
    pub fn me(&self) -> Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let s = state.borrow();
            let url = format!("{}{}", s.base_url, "/user/me");
            let opts = RequestInit::new();
            opts.set_method("GET");
            opts.set_mode(RequestMode::Cors);
            opts.set_credentials(RequestCredentials::Include);
            let req = web_sys::Request::new_with_str_and_init(&url, &opts).map_err(|e| e)?;
            build_headers(&s, true, true, &req).map_err(|e| e)?;
            let res = fetch_json(&req).await?;
            Ok(res)
        })
    }

    #[wasm_bindgen]
    pub fn update(
        &self,
        username: Option<String>,
        email: Option<String>,
        password: Option<String>,
    ) -> Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let s = state.borrow();
            let url = format!("{}{}", s.base_url, "/user/me");
            let mut json = serde_json::json!({});
            if let Some(u) = username {
                json["username"] = serde_json::json!(u);
            }
            if let Some(e) = email {
                json["email"] = serde_json::json!(e);
            }
            if let Some(p) = password {
                json["password"] = serde_json::json!(p);
            }
            let body = json.to_string();
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
    pub fn delete(&self) -> Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let s = state.borrow();
            let url = format!("{}{}", s.base_url, "/user/me");
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
    pub fn get_by_email(&self, email: String) -> Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let s = state.borrow();
            let url = format!("{}{}{}", s.base_url, "/user/", email);
            let opts = RequestInit::new();
            opts.set_method("GET");
            opts.set_mode(RequestMode::Cors);
            opts.set_credentials(RequestCredentials::Include);
            let req = web_sys::Request::new_with_str_and_init(&url, &opts).map_err(|e| e)?;
            let res = fetch_json(&req).await?;
            Ok(res)
        })
    }
}
