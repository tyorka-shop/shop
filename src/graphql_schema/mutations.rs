use crate::{
    entity::Order,
    services::{OrderService, OrderServiceMethods, Recaptcha, RecaptchaError, RecaptchaMethods},
};
use async_graphql::{Context, Error, Object, Result};
use log::error;

use super::order_input::OrderInput;

pub struct Mutations;

#[Object]
impl Mutations {
    async fn add_order<'a>(&self, ctx: &Context<'a>, order: OrderInput) -> Result<Order> {
        let order_service = ctx.data::<OrderService>().unwrap();
        let recaptcha = ctx.data::<Recaptcha>().unwrap();

        let is_valid = match recaptcha.verify(&order.captcha).await {
            Ok(is_valid) => is_valid,
            Err(e) => {
                match e {
                    RecaptchaError::HttpError(e) => error!("Can not verify captcha {:?}", e),
                    RecaptchaError::InvalidResponse(e) => {
                        error!("Invalid captcha response: {:?}", e)
                    }
                };
                return Err(Error::new(format!("Can not check captcha")));
            }
        };

        if !is_valid {
            return Err(Error::new(format!("Invalid captcha")));
        }

        match order_service
            .create_order(
                order.recipient.into(),
                order.cart.into_iter().map(|item| item.id.clone()).collect(),
            )
            .await
        {
            Ok(order) => Ok(order),
            Err(e) => Err(Error::new(format!("{}", e))),
        }
    }
}
