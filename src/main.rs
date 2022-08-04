use actix_web::{self, guard, middleware, web, App, HttpServer};
use actix_web_grants::permissions::AuthDetails;
use actix_web_grants::GrantsMiddleware;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use grant::{RoleData, Role};
use std::io;

mod config;
mod grant;
mod queries;
mod api;

use crate::grant::extract;
use crate::queries::Query;
use crate::api::ApiClient;

async fn index(
    schema: web::Data<Schema<Query, EmptyMutation, EmptySubscription>>,
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
        let api_client = ApiClient::build(&cfg.api_client.url, &cfg.api_client.token).unwrap();
        let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
            .data(api_client)
            .finish();
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
