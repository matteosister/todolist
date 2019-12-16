use seed::{*, prelude::*};

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

#[allow(irrefutable_let_patterns)]
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::HomeMessage(home_msg) => {
            if let Model::Home(home_model) = model {
                page::home::update(home_msg, home_model, &mut orders.proxy(Msg::HomeMessage));
            }
        }
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
