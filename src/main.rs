#![allow(non_snake_case)]
pub mod bakery;
pub mod gym;
pub mod utils;
use bakery::page::*;
use dioxus::prelude::*;
use dioxus_router::{Link, Route, Router};
use gloo::events::EventListener;
use gym::page::*;

fn main() {
    let window = web_sys::window().unwrap_or_else(|| {
        web_sys::console::log_1(&String::from("FATAL error getting window handle!").into());
        panic!();
    });

    let bakery_listener = EventListener::new(&window, "rs-load", move |_ev| {
        bakery::scripts::main();
    });
    bakery_listener.forget();

    let gym_listener = EventListener::new(&window, "rs-load-gym", move |_ev| {
        gym::scripts::main();
    });
    gym_listener.forget();

    dioxus_web::launch(App)
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            HeaderBar {},
            Route { to: "/", HomePage {} },
            Route { to: "/rainbows", Rainbows {} },
            Route { to: "/bakery", BakeryPage {} },
            Route { to: "/bakery/order", OrderPage {} },
            Route { to: "/gym", GymPage {} }
            Route { to: "",  PageNotFound {}}
        }
    })
}

fn Rainbows(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "text-zinc-100 px-96 pt-16 font-mono text-lg",
            div {
                class: "mx-auto",
                h1 {
                    class: "text-6xl text-red-300",
                    "Rainbow Facts!", 
                }
				br {}

                h2 {
                    class: "text-3xl text-amber-300",
                    "Colours of the Rainbow"
                }
				p {
                    "The 7 colours of the rainbow are:"
                        ul {
                            li { class: "text-red-300", "- red" },
                            li { class: "text-orange-300", "- orange" },
                            li { class: "text-amber-300", "- yellow" },
                            li { class: "text-green-300", "- green" },
                            li { class: "text-sky-300", "- blue" },
                            li { class: "text-indigo-300", "- indigo" },
                            li { class: "text-violet-300", "- violet" }
                        }
                }
                br {}

                h2 {
                    class: "text-3xl text-green-300",
                    "What Makes a Rainbow?"
                }
				p {
					"A rainbow is an arch of colours visible in the sky, caused by the refraction and dispersion of the sun's light by rain or other water droplets in the atmosphere. So to see a rainbow, you need mixed weather with a bit of sunshine and a bit of rain too!"
				}
				br {}

                h2 {
                    class: "text-3xl text-sky-300",
                    "Rainbow Myths"
                }
				p {
					"There are several myths related to rainbows. Here is the most popular rainbow myth:",
					p {
						class: "text-2xl font-bold text-amber-300 font-italic",
						"There's a pot of gold at the rainbow's end!"
					}
				}
				br {}

                h2 {
                    class: "text-3xl text-violet-300",
                    "Find Out More"
                }
				p {
					"Check the wikipedia page about rainbows to find out more:",
				}
				a {
					href: "https://en.wikipedia.org/wiki/Rainbow",
					class: "text-zinc-100 underline hover:text-sky-300",
					"https://en.wikipedia.org/wiki/Rainbow"
				}
            }
        }
    })
}

fn PageNotFound(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
            class: "px-96 font-mono text-green-300 mb-0 pb-0",
            style: "font-size: 180px;",
            "404..."
        },
        Link {
            class: "pt-0 mt-0 px-96 font-mono text-zinc-100 hover:text-sky-300 underline text-4xl top-[-120px]",
            to: "/",
            "Go Home"
        }
    })
}

fn HeaderBar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-row px-4 py-4 bg-zinc-900 text-lg font-mono text-zinc-100 justify-center p-0 mb-0",
            id: "header-bar",
            ul {
                Link { class: "p-2 hover:text-sky-300", to: "/", "Home"},
                Link { class: "p-2 hover:text-sky-300", to: "/rainbows", "Rainbows" },
				Link { class: "p-2 hover:text-sky-300", to: "/bakery", "Bakery" },
                Link { class: "p-2 hover:text-sky-300", to: "/gym", "Gym" }
            }
        }
    })
}

fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "font-mono text-4xl mt-16 flex flex-col",
            div {
                class: "mx-auto text-center",
				h1 {
					class: "text-violet-300 text-6xl pb-4",
					"HTML Tasks"
				}
                ul {
					class: "px-6 border-solid border-zinc-100 border-2 py-2",
                    li {
						class: "mx-auto",
						Link { class: "text-zinc-100 underline hover:text-sky-300", to: "/bakery", "Bakery"},
					},
					li {
						class: "mx-auto",
						Link { class: "text-zinc-100 underline hover:text-sky-300", to: "/rainbows", "Rainbows"},
					},
                }
            }
        }
    })
}

//fn Headers(cx: Scope) -> Element {}
