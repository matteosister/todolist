use seed::{*, prelude::*};

use crate::page::ViewPage;

// Model
#[derive(Debug)]
pub struct Model {
    pub title: String,
}

#[derive(Clone, Debug)]
pub enum Msg {}

impl Default for Model {
    fn default() -> Self {
        Self {
            title: "Not Found".to_string()
        }
    }
}

pub fn update(_msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {}


pub fn view(model: &Model) -> ViewPage<Msg> {
    ViewPage::new(model.title.clone(), view_content(model))
}

fn view_content(_model: &Model) -> Node<Msg> {
    div![
        div!["Page not found :("],
        div![a!["Homepage", attrs!{At::Href => "/"}]]
    ]
}