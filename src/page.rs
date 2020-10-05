use seed::prelude::*;

use crate::Msg;

pub mod homepage;
pub mod list;
pub mod not_found;

pub struct ViewPage<Ms: 'static> {
    title_prefix: String,
    content: Node<Ms>,
}

impl<Ms> ViewPage<Ms> {
    pub fn new(title: &str, content: Node<Ms>) -> Self {
        Self {
            title_prefix: title.to_string(),
            content,
        }
    }

    pub fn title(&self) -> String {
        format!("{} - Todolist", self.title_prefix)
    }

    pub fn into_content(self) -> Node<Ms> {
        self.content
    }
}

pub enum Page {
    Home(homepage::Model),
    List(list::Model),
    NotFound(not_found::Model),
}

impl Page {
    pub fn init(url: Url) -> Self {
        match url.path().iter().map(|p| p.as_str()).collect::<Vec<&str>>().as_slice() {
            [] => Self::Home(Default::default()),
            ["list"] => Self::List(Default::default()),
            _ => Self::NotFound(Default::default())
        }
    }

    pub fn view(&self) -> Node<Msg> {
        match self {
            Page::Home(home_model) => {
                let view_page = homepage::view(home_model);
                seed::document().set_title(&view_page.title());
                view_page.into_content().map_msg(Msg::HomeMessage)
            }
            Page::List(list_model) => {
                let view_page = list::view(list_model);
                seed::document().set_title(&view_page.title());
                view_page.into_content().map_msg(Msg::ListMessage)
            }
            Page::NotFound(not_found_model) => {
                let view_page = not_found::view(not_found_model);
                seed::document().set_title(&view_page.title());
                view_page.into_content().map_msg(Msg::NotFoundMessage)
            }
        }
    }
}
