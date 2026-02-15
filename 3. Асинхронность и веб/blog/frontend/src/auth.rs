use crate::{StateRef, utils::fetch_json};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{RequestCredentials, RequestInit, RequestMode};

#[wasm_bindgen]
pub struct Auth {
    pub(crate) state: StateRef,
}

#[wasm_bindgen]
impl Auth {
    #[wasm_bindgen]
    pub fn register(&self, username: String, email: String, password: String) -> Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let mut s = state.borrow_mut();
            let url = format!("{}{}", s.base_url, "/auth/register");
            let body =
                serde_json::json!({"username": username, "email": email, "password": password})
                    .to_string();
            let opts = RequestInit::new();
            opts.set_method("POST");
            opts.set_mode(RequestMode::Cors);
            opts.set_credentials(RequestCredentials::Include);
            opts.set_body(&JsValue::from_str(&body));
            let req = web_sys::Request::new_with_str_and_init(&url, &opts).map_err(|e| e)?;
            let _ = req
                .headers()
                .set("Content-Type", "application/json")
                .map_err(|e| e)?;
            let res = fetch_json(&req).await?;
            if let Ok(at) = js_sys::Reflect::get(&res, &JsValue::from_str("access_token")) {
                if !at.is_undefined() {
                    s.jwt = at.as_string();
                }
            }
            if let Ok(rt) = js_sys::Reflect::get(&res, &JsValue::from_str("refresh_token")) {
                if !rt.is_undefined() {
                    s.refresh = rt.as_string();
                }
            }
            let user =
                js_sys::Reflect::get(&res, &JsValue::from_str("user")).unwrap_or(JsValue::NULL);
            Ok(user)
        })
    }

    #[wasm_bindgen]
    pub fn login(&self, email: String, password: String) -> Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let mut s = state.borrow_mut();
            let url = format!("{}{}", s.base_url, "/auth/login");
            let body = serde_json::json!({"email": email, "password": password}).to_string();
            let opts = RequestInit::new();
            opts.set_method("POST");
            opts.set_mode(RequestMode::Cors);
            opts.set_credentials(RequestCredentials::Include);
            opts.set_body(&JsValue::from_str(&body));
            let req = web_sys::Request::new_with_str_and_init(&url, &opts).map_err(|e| e)?;
            req.headers()
                .set("Content-Type", "application/json")
                .map_err(|e| e)?;
            let res = fetch_json(&req).await?;
            if let Ok(at) = js_sys::Reflect::get(&res, &JsValue::from_str("access_token")) {
                if !at.is_undefined() {
                    s.jwt = at.as_string();
                }
            }
            if let Ok(rt) = js_sys::Reflect::get(&res, &JsValue::from_str("refresh_token")) {
                if !rt.is_undefined() {
                    s.refresh = rt.as_string();
                }
            }
            let user =
                js_sys::Reflect::get(&res, &JsValue::from_str("user")).unwrap_or(JsValue::NULL);
            Ok(user)
        })
    }

    #[wasm_bindgen]
    pub fn logout(&self) -> Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let mut s = state.borrow_mut();
            let url = format!("{}{}", s.base_url, "/auth/logout");
            let payload =
                serde_json::json!({"refresh_token": s.refresh.clone().unwrap_or_default()})
                    .to_string();
            let opts = RequestInit::new();
            opts.set_method("POST");
            opts.set_mode(RequestMode::Cors);
            opts.set_credentials(RequestCredentials::Include);
            opts.set_body(&JsValue::from_str(&payload));
            let req = web_sys::Request::new_with_str_and_init(&url, &opts).map_err(|e| e)?;
            req.headers()
                .set("Content-Type", "application/json")
                .map_err(|e| e)?;
            req.headers().set("x-csrf-token", &s.csrf).map_err(|e| e)?;
            if let Some(token) = &s.jwt {
                req.headers()
                    .set("Authorization", &format!("Bearer {}", token))
                    .map_err(|e| e)?;
            }
            let _ = fetch_json(&req).await?;
            s.jwt = None;
            s.refresh = None;
            Ok(JsValue::UNDEFINED)
        })
    }

    #[wasm_bindgen]
    pub fn refresh(&self) -> Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let mut s = state.borrow_mut();
            let url = format!("{}{}", s.base_url, "/auth/refresh");
            let payload =
                serde_json::json!({"refresh_token": s.refresh.clone().unwrap_or_default()})
                    .to_string();
            let opts = RequestInit::new();
            opts.set_method("POST");
            opts.set_mode(RequestMode::Cors);
            opts.set_credentials(RequestCredentials::Include);
            opts.set_body(&JsValue::from_str(&payload));
            let req = web_sys::Request::new_with_str_and_init(&url, &opts).map_err(|e| e)?;
            req.headers()
                .set("Content-Type", "application/json")
                .map_err(|e| e)?;
            let res = fetch_json(&req).await?;
            if let Ok(at) = js_sys::Reflect::get(&res, &JsValue::from_str("access_token")) {
                if !at.is_undefined() {
                    s.jwt = at.as_string();
                }
            }
            if let Ok(rt) = js_sys::Reflect::get(&res, &JsValue::from_str("refresh_token")) {
                if !rt.is_undefined() {
                    s.refresh = rt.as_string();
                }
            }
            let token = js_sys::Reflect::get(&res, &JsValue::from_str("access_token"))
                .unwrap_or(JsValue::NULL);
            Ok(token)
        })
    }
}
