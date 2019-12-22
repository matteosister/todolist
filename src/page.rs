use seed::prelude::*;

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
    Home,
    List,
    NotFound,
}

impl Page {
    pub fn view<Ms>(&self, view_page: ViewPage<Ms>) -> Vec<Node<Ms>> {
        seed::document().set_title(&view_page.title());

        vec![view_page.into_content()]
    }
}
