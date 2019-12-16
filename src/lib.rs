use seed::{prelude::*, *};

pub mod page;

// Model

enum Model {
    Home(page::home::Model),
}

impl Default for Model {
    fn default() -> Self {
        Model::Home(Default::default())
    }
}

// Update

#[derive(Clone)]
enum Msg {
    HomeMessage(page::home::Msg),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::HomeMessage(_) => model,
    };
}

// View

fn view(model: &Model) -> impl View<Msg> {
    match model {
        Model::Home(home_model) => page::Page::Home
            .view(page::home::view(home_model))
            .map_message(Msg::HomeMessage),
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
