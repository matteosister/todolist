use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Film {
    title: String,
}

impl Film {
    pub fn title(&self) -> &str {
        self.title.as_str()
    }
}
