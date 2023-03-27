#![allow(non_snake_case)]
use crate::gym::scripts;
use crate::utils::*;
use dioxus::prelude::*;
use sha256;
use wasm_bindgen::JsCast;

pub fn GymPage(cx: Scope) -> Element {
    let email = use_state(&cx, || String::new());
    let password = use_state(&cx, || String::new());
    let has_logged_in = use_state(&cx, || scripts::UserDetails::init_none());
    cx.render(rsx! {
        script {
            src: "shim.js"
        }
        match has_logged_in.get() {
            None => rsx!{
				div {
					class: "w-full h-full fill-page bg-zinc-900 flex flex-col p-6",
					id: "main-pre-login",
					div {
						class: "mx-auto",
						id: "gym-title",
						h1 {
							class: "font-bebas-neue text-[120px] text-zinc-50 pt-32",
							"North Herts ",
							span {
								class: "text-lime-400",
								"Gym"
							}
						}
					}
					div {
						class: "mx-auto rounded-xl bg-zinc-50 p-6 center mt-6 flex flex-col",
						id: "gym-login",
						h2 {
							class: "font-bebas-neue text-6xl px-40",
							"Login"
						}
						p {
							class: "font-montserrat font-bold p-2 text-zinc-900",
							"Email Address"
						}
						input {
							class: "w-full rounded-lg text-zinc-900 bg-zinc-200 font-montserrat p-2",
							r#type: "text",
							oninput: move |ev| {
								email.set(ev.value.clone())
							}
						}
						p {
							class: "font-montserrat font-bold p-2 text-zinc-900",
							"Password"
						}
						input {
							class: "w-full rounded-lg text-zinc-900 bg-zinc-200 font-montserrat p-2",
							r#type: "password",
							oninput: move |ev| {
								password.set(sha256::digest(ev.value.clone()))
							}
						}
						br {}
						button {
							class: "bg-lime-400 p-4 rounded-lg mt-4 hover:bg-lime-300 font-montserrat text-center mx-auto center text-zinc-900 font-bold",
							onclick: move |_ev| {
								let mut window = Window::new();
								let console = Console::new();
								console.log(format!("Header bar height: {}", window.document().get_element_by_id("header-bar").unwrap().dyn_into::<web_sys::HtmlElement>().unwrap().offset_height()));
								match scripts::validate(email.get().clone(), password.get().clone()) {
									Ok(user_details) => {
										scripts::handle_page_transition();
										has_logged_in.set(Some(user_details));
									},
									Err(err) => {
										match err {
											scripts::LoginError::InvalidUsername => {
												window.get().alert_with_message("Invalid username!".into()).unwrap();
											},
											scripts::LoginError::InvalidPassword => {
												window.get().alert_with_message("Password incorrect!".into()).unwrap();
											}
										}
									}
								};
							},
							"Go"
						}
					}
                }
            }
			,
            Some(login_data) => {
                rsx! {
					div {
						class: "w-full h-full fill-page bg-zinc-900 flex flex-col",
						div {
							class: "flex flex-col",
							h2 {
								class: "font-bebas-neue text-[150px] text-zinc-50 pl-32 pt-32",
								"Welcome,",
								br {},
								span {
									class: "text-lime-400 relative top-[-80px]",
									match has_logged_in.get() {
										Some(user_data) => rsx! {
											user_data.first_name.clone()
										},
										None => rsx! { "" }
									}
								}
							}
						}
					}
				}
            }
        }
    })
}
