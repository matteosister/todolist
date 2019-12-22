use seed::prelude::*;

use super::prelude::*;

#[derive(Clone, Debug)]
pub enum Route {
    Homepage,
    List,
    NotFound,
}

pub fn routes(url: Url) -> Option<Msg> {
    if url.path.eq(&["".to_string()]) {
        return Some(Msg::ChangePage(Route::Homepage));
    }

    Some(match url.path[0].as_ref() {
        "list" => Msg::ChangePage(Route::List),
        _ => Msg::ChangePage(Route::NotFound),
    })
}
