use actix_cors::Cors;
use actix_web::{self, guard, middleware, web, App, HttpResponse, HttpServer};
use actix_web_grants::permissions::AuthDetails;
use actix_web_grants::GrantsMiddleware;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use grant::{Role, RoleData};
use sea_orm::Database;
use std::io;

mod api;
// mod config;
mod cache;
mod entity;
mod grant;
mod graphql_schema;
mod services;


use config;

use crate::api::{ApiClient, GraphQLClient};
use crate::grant::extract;
use graphql_schema::{build_schema, GQLSchema};
use services::{OrderService, OrderServiceMethods, Recaptcha, RecaptchaMethods, TgBot, TgBotExt};

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
    let cfg = config::load("tyorka-shop".into());
    let port = cfg.port.clone();

    let db = Database::connect(&cfg.database_uri)
        .await
        .expect("Could not connect to database");

    HttpServer::new(move || {
        let auth = GrantsMiddleware::with_extractor(extract);
        let api_client = ApiClient::new(&cfg.api_client).unwrap();
        let order_service =
            OrderService::new(db.clone(), api_client.clone(), TgBot::new(&cfg.tg_client));
        let recaptcha = Recaptcha::new(&cfg.recaptcha);

        let schema = build_schema()
            .data(api_client)
            .data(order_service)
            .data(recaptcha)
            .finish();

        let mut cors = Cors::default()
            .allowed_methods(vec!["POST"])
            .allow_any_header()
            .max_age(3600);

        for origin in cfg.cors_allowed_origins.iter() {
            cors = cors.allowed_origin(origin);
        }

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .wrap(auth)
            .app_data(web::Data::new(cfg.clone()))
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(index))
            .service(
                web::resource("/graphql")
                    .guard(guard::Options())
                    .to(|| HttpResponse::Ok()),
            )

    })
    .bind(format!("0.0.0.0:{port}", port = port))?
    .run()
    .await
}
