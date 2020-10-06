use seed::{prelude::*, *};
use serde::Deserialize;

use crate::prelude::*;

const URL: &str = "https://swapi-graphql.netlify.app/.netlify/functions/index";

#[derive(Debug)]
pub enum Msg {
    LoadFilms,
    FetchedFilms(fetch::Result<GqlResponse>),
}

#[derive(Deserialize, Clone, Debug)]
pub struct GqlResponse {
    data: AllFilmResponse,
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

#[derive(Deserialize, Clone, Debug)]
struct Film {
    title: String,
}

pub struct Model {
    title: String,
    results: Option<fetch::Result<GqlResponse>>,
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
            orders.skip().perform_cmd({
                async { Msg::FetchedFilms(send_message(all_films_query().unwrap()).await) }
            });
        }
        Msg::FetchedFilms(result) => model.results = Some(result),
    }
}

async fn send_message(body: serde_json::Value) -> fetch::Result<GqlResponse> {
    Request::new(URL)
        .method(Method::Post)
        .json(&body)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub fn view(model: &Model) -> ViewPage<Msg> {
    ViewPage::new(model.title.as_str(), view_content(model))
}

fn view_content(model: &Model) -> Node<Msg> {
    section![
        h1!["Star Wars API"],
        div![button!["Load Films", ev(Ev::Click, |_| Msg::LoadFilms)]],
        model.results.as_ref().map_or(div!["cazzo"], view_results)
    ]
}

fn view_results(results: &fetch::Result<GqlResponse>) -> Node<Msg> {
    match results {
        Ok(results) => div![results.data.all_films.edges.iter().map(view_film)],
        _ => div!["errore!"],
    }
}

fn view_film(film_node: &FilmNode) -> Node<Msg> {
    div![h2![film_node.node.title.clone()]]
}
