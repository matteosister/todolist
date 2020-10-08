use seed::{prelude::*, *};
use serde::Deserialize;

use crate::prelude::*;

#[derive(Debug)]
pub enum Msg {
    LoadFilms,
    FetchedFilms(fetch::Result<AllFilmResponse>),
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AllFilmResponse {
    all_films: AllFilmEdge,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AllFilmEdge {
    edges: Vec<FilmNode>,
}

#[derive(Deserialize, Clone, Debug)]
struct FilmNode {
    node: Film,
}

pub struct Model {
    title: String,
    results: Option<fetch::Result<AllFilmResponse>>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            title: "Star Wars API".to_string(),
            results: None,
        }
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::LoadFilms => {
            orders
                .skip()
                .perform_cmd(grapqhl_request(all_films_query()).map(Msg::FetchedFilms));
        }
        Msg::FetchedFilms(result) => model.results = Some(result),
    }
}

pub fn view(model: &Model) -> ViewPage<Msg> {
    ViewPage::new(model.title.as_str(), view_content(model))
}

fn view_content(model: &Model) -> Node<Msg> {
    section![
        h1!["Star Wars API"],
        div![button!["Load Films", ev(Ev::Click, |_| Msg::LoadFilms)]],
        model
            .results
            .as_ref()
            .map_or(div!["chiamata api non ancora effettuata"], view_results)
    ]
}

fn view_results(results: &fetch::Result<AllFilmResponse>) -> Node<Msg> {
    match results {
        Ok(results) => div![results.all_films.edges.iter().map(view_film)],
        _ => div!["errore!"],
    }
}

fn view_film(film_node: &FilmNode) -> Node<Msg> {
    div![h2![film_node.node.title()]]
}
