#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::Link;

pub fn BakeryPage(cx: Scope) -> Element {
    cx.render(rsx! {
            script {
                src: "shim.js"
            }
           div {
               class: "bg-orange-100 w-full h-full fill-page mt-0 flex flex-col",
               div {
                   class: "mx-auto pt-12",
                   span {
                        class: "text-6xl font-playfair font-bold text-center",
                        id: "title",
                        "Belinda's Bakery"
                   }
                   p {
                        class: "text-2xl font-playfair pt-6 text-center",
                        "Cakes and buns, ",
                        span {
                            class: "italic",
                            "for all your special occasions"
                        }
                   }
                   br {}
               }
            div {
                   class: "mx-auto center",
                   ul {
                        class: "pt-10 text-2xl font-playfair mx-auto",
                        li {
                            "🟆 Iced Buns"
                        },
                        li {
                            "🟆 Danish Pastries"
                        },
                        li { "🟆 Birthday Cakes" },
                        li { "🟆 Cupcakes" }
                   }
            },
            Link {
                class: "text-4xl font-bold font-playfair mx-auto pt-6 fx-move",
                id: "order-now",
                to: "/bakery/order",
                "Order Now ->"
            }
        }
    })
}

#[derive(PartialEq, Props)]
pub struct OrderConfirmationProps {
    num_buns: i64,
    num_pastries: i64,
    num_cupcakes: i64,
}

pub fn OrderConfirmationPage(cx: Scope<OrderConfirmationProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "bg-orange-100 w-full- h-full fill-page mt-0 flex flex-col font-playfair",
            h1 {
                class: "text-6xl font-bold mx-auto center pt-12",
                "Order Summary"
            }
            div {
                class: "mx-auto p-4",
                h2 {
                    class: "text-5xl",
                    span {
                        class: "font-bold",
                        "{cx.props.num_buns}"
                    },
                    " Iced Buns"
                }
                h2 {
                    class: "text-5xl pt-2",
                    span {
                        class: "font-bold",
                        "{cx.props.num_pastries}"
                    },
                    " Danish Pastries"
                }
                h2 {
                    class: "text-5xl pt-2",
                    span {
                        class: "font-bold",
                        "{cx.props.num_cupcakes}"
                    },
                    " Cupcakes"
                }
            }
        }
    })
}

pub fn OrderPage(cx: Scope) -> Element {
    let has_ordered = use_state(&cx, || false);
    let num_buns = use_state(&cx, || 0 as i64);
    let num_pastries = use_state(&cx, || 0 as i64);
    let num_cupcakes = use_state(&cx, || 0 as i64);
    cx.render(rsx! {
        if *has_ordered.get() {
            let buns: i64 = *num_buns.get();
            let pastries: i64 = *num_pastries.get();
            let cupcakes: i64 = *num_cupcakes.get();
            rsx!{
				OrderConfirmationPage {
					num_buns: buns,
					num_pastries: pastries,
					num_cupcakes: cupcakes
				}
                p {
                    "{num_buns}",
                    br {},
                    "{num_pastries}",
                    br {},
                    "{num_cupcakes}"
                }
            }
        } else {
            rsx!{div {
               class: "bg-orange-100 w-full h-full fill-page mt-0 flex flex-col font-playfair",
			   h1 {
				   class: "text-6xl font-bold center mx-auto pt-12 pb-6",
				   "Order Now"
			   }
               form {
                   class: "mx-auto center text-2xl",
				   action: "",
                   label {
                        r#for: "icedbuns",
                        "Iced Buns",
                        br {}
                   }
                    input {
                        r#type: "number",
						max: "30",
						value: "0",
                        class: "p-1 m-1",
                        id: "icedbuns",
                        oninput: move |evt| num_buns.set(evt.value.clone().parse().unwrap_or_else(|_| 0))
                    },
                    br {}
                    label {
                        r#for: "danishpastries",
                        "Danish Pastries",
                        br {}
                   }
                    input {
                        r#type: "number",
                        id: "danishpastries",
						max: "30",
						value: "0",
                        class: "p-1 m-1",
                        oninput: move |evt| num_pastries.set(evt.value.clone().parse().unwrap_or_else(|_| 0)),
                        "Danish Pastries",
                        br {}
                    },
                    br {}
                    label {
                        r#for: "cupcakes",
                        "Cupcakes",
                        br {}
                   }
                    input {
                        r#type: "number",
						max: "30",
						value: "0",
                        id: "cupcakes",
                        class: "p-1 m-1",
                        oninput: move |evt| num_cupcakes.set(evt.value.clone().parse().unwrap_or_else(|_| 0)),
                        "Cupcakes",
                        br {}
                    }
                    br {}
                    button {
                        class: "mx-auto mt-6 center",
                        onclick: move |_evt| has_ordered.set(true),
                        p {
                            class: "p-2 mx-auto bg-zinc-900 text-zinc-100 font-bold italic rounded-lg",
                            "Submit"
                        }
                    }
               }
            }}
        }
    })
}
