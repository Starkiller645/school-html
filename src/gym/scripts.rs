use crate::utils::*;
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;

pub fn main() {
    let console = Console::new();
}

pub enum LoginError {
    InvalidUsername,
    InvalidPassword,
}

pub struct UserDetails {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
}

impl UserDetails {
    pub fn init(
        email: impl Into<String>,
        first_name: impl Into<String>,
        last_name: impl Into<String>,
        password_hash: impl Into<String>,
    ) -> Self {
        UserDetails {
            email: email.into(),
            first_name: first_name.into(),
            last_name: last_name.into(),
            password_hash: password_hash.into(),
        }
    }

    pub fn init_none() -> Option<UserDetails> {
        None
    }
}

pub fn validate(user: String, password: String) -> Result<UserDetails, LoginError> {
    let user_map: HashMap<String, UserDetails> = HashMap::from([
        (
            "tallie.tye@outlook.com".into(),
            UserDetails::init(
                "tallie.tye@outlook.com",
                "Tallie",
                "Tye",
                sha256::digest("Passw0rd"),
            ),
        ),
        (
            "17tyet@jhn.herts.sch.uk".into(),
            UserDetails::init(
                "17tyet@jhn.herts.sch.uk",
                "Tallie",
                "Tye",
                sha256::digest("Sch00lp@assword"),
            ),
        ),
    ]);
    match user_map.get(&user) {
        Some(user_info) => {
            if password == user_info.password_hash {
                Ok(UserDetails::init(
                    user_info.email.clone(),
                    user_info.first_name.clone(),
                    user_info.last_name.clone(),
                    user_info.password_hash.clone(),
                ))
            } else {
                Err(LoginError::InvalidPassword)
            }
        }
        None => Err(LoginError::InvalidUsername),
    }
}

pub fn handle_page_transition() {
    let mut window = Window::new();
    let document = window.document();

    document.get_element_by_id("main-pre-login")
        .unwrap()
        .class_list()
        .add_1("animate-out")
        .unwrap();

    spawn_local(async move {
        TimeoutFuture::new(300).await;
        let mut window = Window::new();
        let document = window.document();
        document.get_element_by_id("main-after-login")
            .unwrap()
            .class_list()
            .add_1("animate-in")
            .unwrap();
    })
}
