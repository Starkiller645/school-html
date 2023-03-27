use gloo::{events, render, timers};
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Console {}

pub struct Window {
    internal_window: web_sys::Window,
    internal_document: web_sys::Document,
}

struct Event {
    order_event_listener_over: events::EventListener,
    listeners: HashMap<String, events::EventListener>,
}

thread_local! {
    static LISTENERS: RefCell<Event> = RefCell::new(Event {
        order_event_listener_over: events::EventListener::new(&web_sys::window().unwrap(), "none", |_ev| {}),
        listeners: HashMap::new()
    })
}

impl Window {
    pub fn new() -> Self {
        let internal_window = web_sys::window().unwrap();
        let internal_document = internal_window.document().unwrap();
        Window {
            internal_window,
            internal_document,
        }
    }

    pub fn add_class_to(&mut self, id: &str, class: &str) {
        self.internal_document
            .get_element_by_id(id)
            .unwrap()
            .class_list()
            .add_1(class)
            .unwrap();
    }

    pub fn remove_class_from(&mut self, id: &str, class: &str) {
        self.internal_document
            .get_element_by_id(id)
            .unwrap()
            .class_list()
            .remove_1(class)
            .unwrap();
    }

    pub fn document(&mut self) -> &mut web_sys::Document {
        &mut self.internal_document
    }

    pub fn get(&mut self) -> &mut web_sys::Window {
        &mut self.internal_window
    }

    pub fn register_class_toggle(
        &mut self,
        id: &'static str,
        class_initial: &'static str,
        class_final: &'static str,
        listener_initial: &'static str,
        listener_final: &'static str,
    ) {
        LISTENERS.with(|obj| {
            obj.borrow_mut().listeners.insert(
                String::from(id) + "-" + class_initial + "-" + class_final + "-0",
                events::EventListener::new(
                    &self.internal_document.get_element_by_id(id).unwrap(),
                    listener_initial,
                    |_ev| {
                        web_sys::window()
                            .unwrap()
                            .document()
                            .unwrap()
                            .get_element_by_id(id)
                            .unwrap()
                            .class_list()
                            .add_1(class_initial)
                            .unwrap()
                    },
                ),
            );

            obj.borrow_mut().listeners.insert(
                String::from(id) + "-" + class_initial + "-" + class_final + "-1",
                events::EventListener::new(
                    &self.internal_document.get_element_by_id(id).unwrap(),
                    listener_final,
                    |_ev| {
                        web_sys::window()
                            .unwrap()
                            .document()
                            .unwrap()
                            .get_element_by_id(id)
                            .unwrap()
                            .class_list()
                            .remove_1(class_initial)
                            .unwrap()
                    },
                ),
            );

            obj.borrow_mut().listeners.insert(
                String::from(id) + "-" + class_initial + "-" + class_final + "-2",
                events::EventListener::new(
                    &self.internal_document.get_element_by_id(id).unwrap(),
                    listener_final,
                    |_ev| {
                        web_sys::window()
                            .unwrap()
                            .document()
                            .unwrap()
                            .get_element_by_id(id)
                            .unwrap()
                            .class_list()
                            .add_1(class_final)
                            .unwrap()
                    },
                ),
            );
            obj.borrow_mut().listeners.insert(
                String::from(id) + "-" + class_initial + "-" + class_final + "-3",
                events::EventListener::new(
                    &self.internal_document.get_element_by_id(id).unwrap(),
                    listener_initial,
                    |_ev| {
                        web_sys::window()
                            .unwrap()
                            .document()
                            .unwrap()
                            .get_element_by_id(id)
                            .unwrap()
                            .class_list()
                            .remove_1(class_final)
                            .unwrap()
                    },
                ),
            );
        });
    }

    pub fn register_hover(
        &mut self,
        id: &'static str,
        class_initial: &'static str,
        class_on_hover: &'static str,
    ) {
        self.register_class_toggle(id, class_initial, class_on_hover, "mouseout", "mouseover");
    }
}

impl Console {
    pub fn new() -> Self {
        Console {}
    }
    pub fn log(&self, text: impl Into<String>) {
        web_sys::console::log_1(&text.into().into());
    }
    fn err(&self, text: impl Into<String>) {
        web_sys::console::error_1(&text.into().into());
    }
}
