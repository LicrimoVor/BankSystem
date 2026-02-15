use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

mod auth;
mod general;
mod post;
mod user;
pub(crate) mod utils;

pub use auth::Auth;
pub use general::General;
pub use post::Post;
pub use user::User;

use crate::utils::get_cookie;

pub(crate) struct State {
    pub base_url: String,
    pub csrf: String,
    pub jwt: Option<String>,
    pub refresh: Option<String>,
}

pub(crate) type StateRef = Rc<RefCell<State>>;

#[wasm_bindgen]
pub struct Api {
    state: StateRef,
}

#[wasm_bindgen]
impl Api {
    #[wasm_bindgen(constructor)]
    pub async fn new(addr: String) -> Api {
        let base = format!("{}/api", addr.trim_end_matches('/'));

        let state = State {
            base_url: base,
            csrf: "".to_string(),
            jwt: None,
            refresh: None,
        };
        let api = Api {
            state: Rc::new(RefCell::new(state)),
        };
        if let Ok(val) = wasm_bindgen_futures::JsFuture::from(api.general().health()).await {
            utils::log(format!("Health: {}", val.as_string().unwrap()).as_str());
        };
        let csrf = get_cookie("csrf-token").await;
        if let Some(csrf) = csrf {
            api.state.borrow_mut().csrf = csrf;
        }

        api
    }

    #[wasm_bindgen]
    pub fn general(&self) -> General {
        General {
            state: self.state.clone(),
        }
    }

    #[wasm_bindgen]
    pub fn auth(&self) -> Auth {
        Auth {
            state: self.state.clone(),
        }
    }

    #[wasm_bindgen]
    pub fn post(&self) -> Post {
        Post {
            state: self.state.clone(),
        }
    }

    #[wasm_bindgen]
    pub fn user(&self) -> User {
        User {
            state: self.state.clone(),
        }
    }
}
