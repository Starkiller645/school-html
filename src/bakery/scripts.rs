use crate::utils::*;

pub fn main() {
    let console = Console::new();
    let mut window = Window::new();
    let pathname = window.get().location().pathname().unwrap();
    console.log(format!("Routing to `{pathname}`..."));
    match pathname.as_str() {
        "/bakery" => route_bakery(&mut window),
        "/bakery/order" => route_bakery_order(&mut window),
        _ => console.log(format!("Current route `{pathname}` not handled")),
    };
}

pub fn route_bakery(window: &mut Window) {
    window.register_hover("order-now", "order-now-initial", "order-now-final");
    window.register_hover("title", "title-initial", "title-final");
}

pub fn route_bakery_order(window: &mut Window) {}
