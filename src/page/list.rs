use seed::{*, prelude::*};

use crate::page::ViewPage;

// Model
#[derive(Debug)]
pub struct Model {
    title: String,
    elements: Vec<String>
}

#[derive(Clone, Debug)]
pub enum Msg {}

impl Default for Model {
    fn default() -> Self {
        Self {
            title: "List".to_string(),
            elements: vec![]
        }
    }
}

pub fn update(_msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {}

// View

pub fn view(model: &Model) -> ViewPage<Msg> {

    ViewPage::new(model.title.as_str(), view_content(model))
}

fn view_content(_model: &Model) -> Node<Msg> {
    div![
        a!["Homepage", attrs!{At::Href => "/"}],
        h1!["List"],
        "list"
    ]
}
