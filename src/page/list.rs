use seed::{prelude::*, *};
use uuid::Uuid;
use web_sys::{Event, KeyboardEvent};

use crate::page::ViewPage;

const ENTER: u32 = 13;

// Model
#[derive(Debug)]
pub struct Model {
    title: String,
    elements: Vec<Element>,
    new_element_label: String,
    active_filter: Filter,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            title: "List".to_string(),
            elements: vec![Element::new("element one"), Element::new("element two")],
            new_element_label: "".to_string(),
            active_filter: Filter::All,
        }
    }
}

impl Model {
    pub fn add_element(&mut self, el: Element) {
        self.elements.push(el);
    }

    pub fn remove_element(&mut self, element_id: Uuid) {
        self.elements.retain(|el| el.id != element_id)
    }

    pub fn remove_completed(&mut self) {
        self.elements.retain(|el| !el.done)
    }

    pub fn toggle_element(&mut self, element_id: Uuid) {
        for el in self.elements.iter_mut() {
            if el.id == element_id {
                el.toggle();
            }
        }
    }

    fn get_actual_elements(&self) -> Vec<&Element> {
        self.get_elements(&self.active_filter)
    }

    fn get_elements(&self, filter: &Filter) -> Vec<&Element> {
        self.elements
            .iter()
            .filter(|element| match filter {
                Filter::All => true,
                Filter::Active => !element.done,
                Filter::Completed => element.done,
            })
            .collect()
    }

    fn count_elements(&self) -> usize {
        self.get_elements(&Filter::Active).len()
    }

    fn has_completed_tasks(&self) -> bool {
        self.elements.iter().any(|el| el.done)
    }
}

#[derive(Debug, Clone)]
pub enum Filter {
    All,
    Active,
    Completed,
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
    KeyPressed(Event),
    DeleteElement(Uuid),
    ToggleElement(Uuid),
    ChangeFilter(Filter),
    RemoveCompleted,
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::EditEntry(val) => {
            model.new_element_label = val;
        }
        Msg::KeyPressed(ev) => {
            let code = ev.unchecked_into::<KeyboardEvent>().key_code();
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
        Msg::ChangeFilter(filter) => {
            model.active_filter = filter;
        }
        Msg::RemoveCompleted => model.remove_completed()
    }
}

// View

pub fn view(model: &Model) -> ViewPage<Msg> {
    ViewPage::new(model.title.as_str(), view_content(model))
}

fn view_content(model: &Model) -> Node<Msg> {
    section![
        class!["todoapp"],
        //        a!["Homepage", attrs! {At::Href => "/"}],
        header![
            h1!["todos"],
            input![
                class!["new-todo"],
                attrs! {At::Value => model.new_element_label, At::Placeholder => "What needs to be done?"},
                input_ev(Ev::Input, Msg::EditEntry),
                ev(Ev::KeyDown, Msg::KeyPressed),
            ]
        ],
        section![
            class!["main"],
            input![
                class!["toggle-all"],
                id!["toggle-all"],
                attrs! {At::Type => "checkbox"},
            ],
            label![attrs! {At::For => "toggle-all"}]
        ],
        view_list(model),
        footer(model),
    ]
}

fn view_list(model: &Model) -> Node<Msg> {
    ul![
        class!["todo-list"],
        model.get_actual_elements().into_iter().map(view_element)
    ]
}

fn view_element(el: &Element) -> Node<Msg> {
    li![
        attrs! {At::Id => el.id, At::Class => if el.done { "completed" } else {""}},
        div![
            class!["view"],
            input![
                class!["toggle"],
                attrs! {At::Type => "checkbox", At::Checked => el.done.as_at_value()},
                simple_ev(Ev::Click, Msg::ToggleElement(el.id))
            ],
            label![el.label.as_str()],
            button![
                class!["destroy"],
                simple_ev(Ev::Click, Msg::DeleteElement(el.id))
            ]
        ]
    ]
}

fn footer(model: &Model) -> Node<Msg> {
    let count = model.count_elements();

    footer![
        class!["footer"],
        span![
            class!["todo-count"],
            format!("{} item{} left", count, if count == 1 { "" } else { "s" })
        ],
        ul![
            class!["filters"],
            li![a![
                attrs! {At::Href => ""},
                simple_ev(Ev::Click, Msg::ChangeFilter(Filter::All)),
                "All"
            ]],
            li![a![
                attrs! {At::Href => ""},
                simple_ev(Ev::Click, Msg::ChangeFilter(Filter::Active)),
                "Active"
            ]],
            li![a![
                attrs! {At::Href => ""},
                simple_ev(Ev::Click, Msg::ChangeFilter(Filter::Completed)),
                "Completed"
            ]]
        ],
        if model.has_completed_tasks() {
            button![class!["clear-completed"], simple_ev(Ev::Click, Msg::RemoveCompleted), "Clear completed"]
        } else {
            div![]
        }
    ]
}
