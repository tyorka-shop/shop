use crate::{
    entity::{Order, Recipient},
    services::{OrderService, OrderServiceMethods, Recaptcha, RecaptchaMethods, RecaptchaError},
};
use async_graphql::{Context, InputObject, Object, Result, Error};

pub struct Mutations;

#[derive(InputObject)]
pub struct ProductInput {
    pub id: String,
    pub count: Option<u32>,
}

#[derive(InputObject)]
pub struct RecipientInput {
    pub name: String,
    pub email: String,
}

#[derive(InputObject)]
pub struct NewOrder {
    pub recipient: RecipientInput,
    pub cart: Vec<ProductInput>,
    pub captcha: String,
}

impl From<NewOrder> for Order {
    fn from(new_order: NewOrder) -> Self {
        Self {
            recipient: Recipient {
                name: new_order.recipient.name,
                email: new_order.recipient.email,
            },
            cart: new_order
                .cart
                .iter()
                .map(|product| product.id.clone())
                .collect(),
        }
    }
}

#[Object]
impl Mutations {
    async fn add_order<'a>(&self, ctx: &Context<'a>, order: NewOrder) -> Result<String> {
        let order_service = ctx.data::<OrderService>().unwrap();
        let recaptcha = ctx.data::<Recaptcha>().unwrap();

        let is_valid = match recaptcha.verify(&order.captcha).await {
            Ok(is_valid) => is_valid,
            Err(e) => {
                match e {
                    RecaptchaError::HttpError(e) => log::error!("Can not verify captcha {:?}", e),
                    RecaptchaError::InvalidResponse(e) => log::error!("Invalid captcha response: {:?}", e),
                };
                return Err(Error::new(format!("Can not check captcha")))
            },
        };

        if !is_valid {
            return Err(Error::new(format!("Invalid captcha")));
        }
        
        match order_service.add_order(order.into()).await {
            Ok(_) => Ok("Ok".into()),
            Err(e) => Err(Error::new(format!("{}", e))),
        }
    }
}
