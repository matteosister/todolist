use seed::prelude::*;

pub mod home;

pub struct ViewPage<Ms: 'static> {
    title_prefix: String,
    content: Node<Ms>,
}

impl<Ms> ViewPage<Ms> {
    pub fn new(title: String, content: Node<Ms>) -> Self {
        Self {
            title_prefix: title,
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
}

impl Page {
    pub fn view<Ms>(&self, view_page: ViewPage<Ms>) -> Vec<Node<Ms>> {
        seed::document().set_title(&view_page.title());

        vec![view_page.into_content()]
    }
}
