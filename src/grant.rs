use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web::web::Data;
use actix_web_grants::permissions::AuthDetails;
use crate::config::Config;

#[derive(PartialEq, Clone)]
pub enum Role {
    Client,
    Admin,
}

fn is_admin(req: &ServiceRequest) -> bool {
    let cfg = req.app_data::<Data<Config>>().unwrap();

    match req.headers().get("x-auth") {
        Some(token) => {
            match token.to_str() {
                Ok(token) => {
                    token.len() == cfg.secret.len() && token == cfg.secret
                }
                Err(_) => false
            }
        },
        None => false
    }
}

pub async fn extract(req: &ServiceRequest) -> Result<Vec<Role>, Error> {
    if is_admin(&req) { Ok(vec![Role::Admin]) } else { Ok(vec![Role::Client]) }
}

pub struct RoleData(pub AuthDetails<Role>);

impl From<&RoleData> for String {
    fn from(role: &RoleData) -> String {
        match role.0.permissions[0] {
            Role::Client => "client".to_string(),
            Role::Admin => "admin".to_string()
        }
    }
}