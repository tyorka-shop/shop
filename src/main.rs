use actix_web::{self, guard, middleware, web, App, HttpServer};
use actix_web_grants::permissions::AuthDetails;
use actix_web_grants::GrantsMiddleware;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use grant::{Role, RoleData};
use std::io;

mod api;
mod config;
mod grant;
mod graphql_schema;

use crate::api::{ApiClient, ApiClientExt};
use crate::grant::extract;
use crate::graphql_schema::{build_schema, GQLSchema};

async fn index(
    schema: web::Data<GQLSchema>,
    request: GraphQLRequest,
    role: AuthDetails<Role>,
) -> GraphQLResponse {
    schema
        .execute(request.into_inner().data(RoleData(role)))
        .await
        .into()
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    let cfg = config::load();
    let port = cfg.port.clone();

    HttpServer::new(move || {
        let auth = GrantsMiddleware::with_extractor(extract);
        let api_client = ApiClient::new(&cfg.api_client.url, &cfg.api_client.token).unwrap();
        let schema = build_schema().data(api_client).finish();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(auth)
            .app_data(web::Data::new(cfg.clone()))
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(index))
    })
    .bind(format!("0.0.0.0:{port}", port = port))?
    .run()
    .await
}
