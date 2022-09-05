use crate::{
    entity::{Order, Store},
    services::{OrderService, OrderServiceMethods, Recaptcha, RecaptchaMethods, TgBot, TgBotExt},
};
use async_graphql::{Context, Object, Result};
use log::error;
use sea_orm::DatabaseConnection;

use super::order_input::OrderInput;

pub struct Mutations;

#[Object]
impl Mutations {
    async fn add_order<'a>(&self, ctx: &Context<'a>, order: OrderInput) -> Result<Order> {
        let order_service = ctx.data::<OrderService>().unwrap();
        let recaptcha = ctx.data::<Recaptcha>().unwrap();
        let tg_bot = ctx.data::<TgBot>().unwrap();
        let db = ctx.data::<DatabaseConnection>().unwrap();

        recaptcha.verify(&order.captcha).await?;

        let order = order_service
            .create_order(
                order.recipient.into(),
                order.cart.into_iter().map(|item| item.id.clone()).collect(),
            )
            .await?;

        order.insert(db).await?;
        
        match tg_bot.send_messages(&order.to_text()).await  {
            Ok(_) => {},
            Err(e) => {
                error!("Tg message does not sent: {}", e);
            }
        }

        Ok(order)
    }
}
