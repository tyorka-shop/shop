use async_trait::async_trait;
use futures::future::join_all;
use log::{error};
use std::fmt;

use crate::api::{ApiClient, ApiMethods, GetProductError};
use crate::entity::Recipient;
use crate::entity::{CartItem, Order};


pub struct OrderService {
    api_client: ApiClient,
}

#[derive(Debug)]
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
        write!(f, "{}", &self.to_string())
    }
}

#[async_trait]
pub trait OrderServiceMethods {
    fn new(api_client: ApiClient) -> Self;
    async fn enhance_cart(&self, cart: &Vec<String>) -> Result<Vec<CartItem>, GetProductError>;
    async fn create_order(
        &self,
        recipient: Recipient,
        cart: Vec<String>,
    ) -> Result<Order, OrderServiceError>;
}

#[async_trait]
impl OrderServiceMethods for OrderService {
    fn new(api_client: ApiClient) -> Self {
        OrderService {
            api_client,
        }
    }
    async fn enhance_cart(&self, cart: &Vec<String>) -> Result<Vec<CartItem>, GetProductError> {
        let cart = join_all(cart.into_iter().map(|id| async {
            self.api_client
                .get_product(&id.clone())
                .await
                .unwrap()
                .into()
        }))
        .await;
        Ok(cart)
    }

    async fn create_order(
        &self,
        recipient: Recipient,
        cart: Vec<String>,
    ) -> Result<Order, OrderServiceError> {
        let cart = match self.enhance_cart(&cart).await {
            Ok(cart) => cart,
            Err(_) => {
                error!("Can not get products");
                return Err(OrderServiceError::GetProductError);
            }
        };

        Ok(Order::new(recipient, cart))
    }
}
