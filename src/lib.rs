use seed::{prelude::*, *};

use prelude::*;

pub mod page;
pub mod prelude;
pub mod route;

// Model
#[derive(Debug)]
enum Model {
    Home(homepage::Model),
    List(list::Model),
    NotFound(not_found::Model),
}

impl Default for Model {
    fn default() -> Self {
        Model::Home(Default::default())
    }
}

// Update

#[derive(Clone, Debug)]
pub enum Msg {
    HomeMessage(homepage::Msg),
    ListMessage(list::Msg),
    NotFoundMessage(not_found::Msg),
    ChangePage(Route),
}

#[allow(irrefutable_let_patterns)]
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match seed::log(msg) {
        Msg::HomeMessage(home_msg) => {
            if let Model::Home(home_model) = model {
                homepage::update(home_msg, home_model, &mut orders.proxy(Msg::HomeMessage));
            }
        }
        Msg::ListMessage(list_msg) => {
            if let Model::List(list_model) = model {
                list::update(list_msg, list_model, &mut orders.proxy(Msg::ListMessage));
            }
        }
        Msg::NotFoundMessage(not_found_msg) => {
            if let Model::NotFound(not_found_model) = model {
                not_found::update(
                    not_found_msg,
                    not_found_model,
                    &mut orders.proxy(Msg::NotFoundMessage),
                );
            }
        }
        Msg::ChangePage(route) => {
            *model = match route {
                Route::Homepage => Model::Home(Default::default()),
                Route::List => Model::List(Default::default()),
                Route::NotFound => Model::NotFound(Default::default()),
            };
        }
    };
}

// View

fn view(model: &Model) -> impl View<Msg> {
    match model {
        Model::Home(home_model) => Page::Home
            .view(page::homepage::view(home_model))
            .map_message(Msg::HomeMessage),
        Model::List(list_model) => Page::List
            .view(page::list::view(list_model))
            .map_message(Msg::ListMessage),
        Model::NotFound(not_found_model) => Page::NotFound
            .view(page::not_found::view(not_found_model))
            .map_message(Msg::NotFoundMessage),
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .routes(self::route::routes)
        .build_and_start();
}
