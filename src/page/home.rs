use crate::page::ViewPage;
use seed::{prelude::*, *};

pub struct Model {
    pub title: String,
}

#[derive(Clone)]
pub enum Msg {}

impl Default for Model {
    fn default() -> Self {
        Self {
            title: "Homepage".into(),
        }
    }
}

// View

pub fn view(model: &Model) -> ViewPage<Msg> {
    ViewPage::new(model.title.clone(), view_content(model))
}

fn view_content(model: &Model) -> Node<Msg> {
    div![class!["home-page"], model.title,]
}
