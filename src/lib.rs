use seed::{*, prelude::*};

use prelude::*;

pub mod page;
pub mod prelude;

// Model
struct Model {
    page: Page,
    base_url: Url,
}


// Update

#[derive(Clone, Debug)]
pub enum Msg {
    UrlChanged(subs::UrlChanged),
    HomeMessage(homepage::Msg),
    ListMessage(list::Msg),
    NotFoundMessage(not_found::Msg),
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    seed::log(&url);
    orders.subscribe(Msg::UrlChanged);
    seed::log(url.to_base_url());
    Model {
        base_url: url.to_base_url(),
        page: Page::init(url),
    }
}

#[allow(irrefutable_let_patterns)]
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(url) => {
            model.page = Page::init(url.0);
        }
        Msg::HomeMessage(home_msg) => {
            if let Page::Home(home_model) = &mut model.page {
                homepage::update(home_msg, home_model, &mut orders.proxy(Msg::HomeMessage));
            }
        }
        Msg::ListMessage(list_msg) => {
            if let Page::List(list_model) = &mut model.page {
                list::update(list_msg, list_model, &mut orders.proxy(Msg::ListMessage));
            }
        }
        Msg::NotFoundMessage(not_found_msg) => {
            if let Page::NotFound(not_found_model) = &mut model.page {
                not_found::update(
                    not_found_msg,
                    not_found_model,
                    &mut orders.proxy(Msg::NotFoundMessage),
                );
            }
        }
    };
}

// View

fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        nav![
            ul![
                li![a!["Homepage", attrs! {At::Href => "/"}]],
                li![a!["List", attrs! {At::Href => "/list"}]]
            ]
        ],
        model.page.view()
    ]
}


#[wasm_bindgen(start)]
pub fn render() {
    //App::builder(update, view).build();

    App::start("app", init, update, view);
}
