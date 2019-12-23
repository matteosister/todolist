use seed::{prelude::*, *};

use crate::page::ViewPage;
use uuid::Uuid;

const ENTER: u32 = 13;

// Model
#[derive(Debug)]
pub struct Model {
    title: String,
    elements: Vec<Element>,
    new_element_label: String,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            title: "List".to_string(),
            elements: vec![Element::new("element one"), Element::new("element two")],
            new_element_label: "".to_string(),
        }
    }
}

impl Model {
    pub fn add_element(&mut self, el: Element) {
        self.elements.push(el);
    }

    pub fn remove_element(&mut self, element_id: Uuid) {
        self.elements = self
            .elements
            .iter()
            .cloned()
            .filter(|el| el.id != element_id)
            .collect();
    }

    pub fn toggle_element(&mut self, element_id: Uuid) {
        self.elements = self
            .elements
            .iter()
            .cloned()
            .map(|mut el| {
                if el.id == element_id {
                    el.toggle();
                }
                el
            })
            .collect();
    }
}

#[derive(Debug, Clone)]
pub struct Element {
    id: Uuid,
    label: String,
    done: bool,
}

impl Element {
    pub fn new(label: impl ToString) -> Self {
        Self {
            id: Uuid::new_v4(),
            label: label.to_string(),
            done: false,
        }
    }

    pub fn toggle(&mut self) {
        seed::log(&self);
        self.done = !self.done;
    }
}

#[derive(Clone, Debug)]
pub enum Msg {
    EditEntry(String),
    KeyPressed(events::Event),
    DeleteElement(Uuid),
    ToggleElement(Uuid),
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::EditEntry(val) => {
            model.new_element_label = val;
        }
        Msg::KeyPressed(ev) => {
            let code = seed::to_kbevent(&ev).key_code();
            if code == ENTER {
                let element = Element::new(model.new_element_label.clone());
                model.add_element(element);
                model.new_element_label = "".to_string();
            }
        }
        Msg::DeleteElement(element_id) => {
            model.remove_element(element_id);
        }
        Msg::ToggleElement(element_id) => model.toggle_element(element_id),
    }
}

// View

pub fn view(model: &Model) -> ViewPage<Msg> {
    ViewPage::new(model.title.as_str(), view_content(model))
}

fn view_content(model: &Model) -> Node<Msg> {
    div![
        a!["Homepage", attrs! {At::Href => "/"}],
        h1!["List"],
        input![
            attrs! {At::Value => model.new_element_label},
            input_ev(Ev::Input, Msg::EditEntry),
            raw_ev(Ev::KeyDown, Msg::KeyPressed),
        ],
        view_list(model)
    ]
}

fn view_list(model: &Model) -> Node<Msg> {
    ul![model.elements.iter().map(view_element)]
}

fn view_element(el: &Element) -> Node<Msg> {
    li![
        attrs! {At::Id => el.id},
        el.label,
        " ",
        a![
            attrs! {At::Href => ""},
            simple_ev(Ev::Click, Msg::DeleteElement(el.id)),
            "remove"
        ],
        " ",
        input![
            attrs! {At::Type => "checkbox", At::Checked => el.done.as_at_value()},
            simple_ev(Ev::Click, Msg::ToggleElement(el.id))
        ]
    ]
}
