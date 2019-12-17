use seed::{*, prelude::*};

use prelude::*;

pub mod page;
pub mod prelude;

// Model
enum Model {
    Home(homepage::Model),
}

impl Default for Model {
    fn default() -> Self {
        Model::Home(Default::default())
    }
}


// Update

#[derive(Clone)]
enum Msg {
    HomeMessage(homepage::Msg),
}

#[allow(irrefutable_let_patterns)]
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::HomeMessage(home_msg) => {
            if let Model::Home(home_model) = model {
                homepage::update(home_msg, home_model, &mut orders.proxy(Msg::HomeMessage));
            }
        }
    };
}

// View

fn view(model: &Model) -> impl View<Msg> {
    match model {
        Model::Home(home_model) => Page::Home
            .view(page::homepage::view(home_model))
            .map_message(Msg::HomeMessage),
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
