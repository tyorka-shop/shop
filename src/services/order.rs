use async_trait::async_trait;
use entity::cart_item;
use entity::order;
use indoc::formatdoc;
use log::{error, info};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use std::fmt;

use crate::api::{ApiClient, ApiMethods, GetProductError};
use crate::entity::{Order, Product};

use super::{TgBot, TgBotExt};

pub struct OrderService {
    db: DatabaseConnection,
    api_client: ApiClient,
    tg_client: TgBot,
}

pub enum OrderServiceError {
    GetProductError,
    SaveError
}

impl From<OrderServiceError> for String {
    fn from(error: OrderServiceError) -> Self {
        match error {
            OrderServiceError::GetProductError => "Can not get products".to_string(),
            OrderServiceError::SaveError => "Can not save order".to_string(),
        }
    }
}

impl fmt::Display for OrderServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderServiceError::GetProductError => write!(f, "Can not get products"),
            OrderServiceError::SaveError => write!(f, "Can not save order"),
        }
    }
}

#[async_trait]
pub trait OrderServiceMethods {
    fn new(db: DatabaseConnection, api_client: ApiClient, tg_client: TgBot) -> Self;
    async fn get_cart(&self, cart: &Vec<String>) -> Result<Vec<Product>, GetProductError>;
    async fn add_order(&self, order: Order) -> Result<(), OrderServiceError>;
    async fn notify(&self, order: Order) -> Result<(), OrderServiceError>;
}

#[async_trait]
impl OrderServiceMethods for OrderService {
    fn new(db: DatabaseConnection, api_client: ApiClient, tg_client: TgBot) -> Self {
        OrderService {
            db,
            api_client,
            tg_client,
        }
    }
    async fn get_cart(&self, cart: &Vec<String>) -> Result<Vec<Product>, GetProductError> {
        let mut products = Vec::new();
        for id in cart {
            let product = self.api_client.get_product(&id).await?;
            products.push(product);
        }
        Ok(products)
    }

    async fn notify(&self, order: Order) -> Result<(), OrderServiceError> {
        let cart = match self.get_cart(&order.cart).await {
            Ok(cart) => cart,
            Err(_) => {
                error!("Can not get products");
                return Err(OrderServiceError::GetProductError);
            }
        };

        let titles = cart
            .iter()
            .map(|product| format!("- {}", product.title.clone()))
            .collect::<Vec<_>>()
            .join("\n");

        let message = formatdoc! {
            "Новый заказ
        От {name} {email}

        {titles}",

            name = order.recipient.name,
            email = order.recipient.email,
            titles = titles
        };

        match self.tg_client.send_messages(&message).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Tg message does not sent: {}", e);
                Ok(())
            }
        }
    }

    async fn add_order(&self, order: Order) -> Result<(), OrderServiceError> {
        info!("Try to add order: {:?}", &order);

        let cart = match self.get_cart(&order.cart).await {
            Ok(cart) => cart,
            Err(_) => {
                error!("Can not get products");
                return Err(OrderServiceError::GetProductError);
            }
        };

        let order_model = order::ActiveModel {
            recipient_name: Set(order.recipient.name.clone()),
            recipient_email: Set(order.recipient.email.clone()),
            ..Default::default()
        };

        let result = order_model.save(&self.db).await.map_err(|_| {
            error!("Can not save order");
            OrderServiceError::SaveError
        })?;

        let order_id = result.id.unwrap();

        for product in cart {
            let cart_item_model = cart_item::ActiveModel {
                order_id: Set(order_id.clone()),
                product_id: Set(product.id.into()),
                price: Set(product.price),
                ..Default::default()
            };
            cart_item_model.save(&self.db).await.map_err(|_| {
                error!("Can not save cart item");
                OrderServiceError::SaveError
            })?;
        }

        info!("Order saved");

        self.notify(order).await?;

        Ok(())
    }
}
