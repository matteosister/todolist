use seed::{*, prelude::*};

use crate::page::ViewPage;

// Model
#[derive(Debug)]
pub struct Model {
    title: String,
    count: i32,
}

#[derive(Clone, Debug)]
pub enum Msg {
    Increment,
    Decrement
}

impl Default for Model {
    fn default() -> Self {
        Self {
            title: "Homepage".into(),
            count: 0,
        }
    }
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.count += 1,
        Msg::Decrement => model.count -= 1,
    }
}

// View
pub fn view(model: &Model) -> ViewPage<Msg> {
    ViewPage::new(model.title.as_str(), view_content(model))
}

fn view_content(model: &Model) -> Node<Msg> {
    div![
        class!["home-page"],
        div!["count: ", model.count.to_string()],
        div![
            button![simple_ev(Ev::Click, Msg::Decrement), "decrement"],
            button![simple_ev(Ev::Click, Msg::Increment), "increment"]
        ],
        a!["List", attrs!{At::Href => "/list"}]
    ]
}
