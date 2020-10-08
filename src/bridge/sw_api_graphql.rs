use graphql_client::*;
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

const URL: &str = "https://swapi-graphql.netlify.app/.netlify/functions/index";

#[derive(Deserialize)]
pub struct GraphQLResponse<T> {
    data: T,
}

pub async fn grapqhl_request<T: 'static, V: Serialize>(
    query: graphql_client::QueryBody<V>,
) -> fetch::Result<T>
where
    for<'de> T: Deserialize<'de>,
{
    let body = serde_json::to_value(query).map_err(|e| FetchError::SerdeError(e));
    let res: fetch::Result<GraphQLResponse<T>> = Request::new(URL)
        .method(Method::Post)
        .json(&body?)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await;

    res.map(|res| res.data)
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "graphql/all_films.graphql",
    schema_path = "graphql/schema.graphql"
)]
struct AllFilms;

pub fn all_films_query() -> graphql_client::QueryBody<all_films::Variables> {
    AllFilms::build_query(all_films::Variables {})
}
