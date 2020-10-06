use graphql_client::*;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "graphql/all_films.graphql",
    schema_path = "graphql/schema.graphql"
)]
struct AllFilms;

pub fn all_films_query() -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&AllFilms::build_query(all_films::Variables {}))
}
