use async_trait::async_trait;
use indoc::formatdoc;
use log::{error, info};
use sea_orm::DatabaseConnection;
use std::fmt;

use crate::api::{ApiClient, ApiMethods, GetProductError};
use crate::entity::{Order, Store};
use crate::entity::{Product, Recipient};

use super::{TgBot, TgBotExt};

pub struct OrderService {
    db: DatabaseConnection,
    api_client: ApiClient,
    tg_client: TgBot,
}

pub enum OrderServiceError {
    GetProductError,
    SaveError,
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
    async fn create_order(
        &self,
        recipient: Recipient,
        cart: Vec<String>,
    ) -> Result<Order, OrderServiceError>;
    async fn notify(&self, order: &Order) -> Result<(), OrderServiceError>;
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

    async fn notify(&self, order: &Order) -> Result<(), OrderServiceError> {
        let titles = order
            .cart
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

    async fn create_order(
        &self,
        recipient: Recipient,
        cart: Vec<String>,
    ) -> Result<Order, OrderServiceError> {
        
        let cart = match self.get_cart(&cart).await {
            Ok(cart) => cart,
            Err(_) => {
                error!("Can not get products");
                return Err(OrderServiceError::GetProductError);
            }
        };
        
        info!("Try to add order {:?} for {:?}", &cart, &recipient);

        let order = Order::new(recipient, cart);

        order
            .insert(&self.db)
            .await
            .map_err(|_| OrderServiceError::SaveError)?;

        info!("Order saved");

        self.notify(&order).await?;

        Ok(order)
    }
}
