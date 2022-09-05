use crate::config::Config;
use actix_web::dev::ServiceRequest;
use actix_web::web::Data;
use actix_web::Error;
use actix_web_grants::permissions::AuthDetails;
use async_graphql::Guard;
use async_graphql::{Context, Result};
use async_trait::async_trait;
use constant_time_eq::constant_time_eq;

#[derive(PartialEq, Clone)]
pub enum Role {
    Client,
    Admin,
}

fn is_admin(req: &ServiceRequest) -> bool {
    let cfg = req.app_data::<Data<Config>>().unwrap();

    match req.headers().get("x-auth") {
        Some(token) => match token.to_str() {
            Ok(token) => constant_time_eq(&token.as_bytes(), &cfg.secret.as_bytes()),
            Err(_) => false,
        },
        None => false,
    }
}

pub async fn extract(req: &ServiceRequest) -> Result<Vec<Role>, Error> {
    match is_admin(&req) {
        true => Ok(vec![Role::Admin]),
        false => Ok(vec![Role::Client]),
    }
}

pub struct RoleData(pub AuthDetails<Role>);

impl RoleData {
    pub fn admin() -> Self {
        RoleData(AuthDetails {
            permissions: vec![Role::Admin],
        })
    }
}

#[async_trait]
impl Guard for RoleData {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let expect: Role = self.into();
        let actual: Role = ctx.data::<RoleData>().unwrap().into();
        if expect != Role::Client && expect != actual {
            return Err("Permission denied".into());
        }
        Ok(())
    }
}

impl From<&RoleData> for String {
    fn from(role: &RoleData) -> String {
        match role.0.permissions[0] {
            Role::Client => "client".to_string(),
            Role::Admin => "admin".to_string(),
        }
    }
}

impl From<&RoleData> for Role {
    fn from(role: &RoleData) -> Role {
        role.0.permissions[0].clone()
    }
}
