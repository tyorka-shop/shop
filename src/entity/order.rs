use super::{Product, Recipient};
use async_graphql::SimpleObject;
use async_trait::async_trait;
use chrono::Utc;
use entity::{cart_item, order};
use log::{error};
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, Set,
};
use uuid::Uuid;

#[derive(SimpleObject, Debug)]
pub struct Order {
    pub id: String,
    pub date: String,
    pub status: String,
    pub recipient: Recipient,
    pub cart: Vec<Product>,
}

pub enum StoreError {
    SaveError,
}

#[async_trait]
pub trait Store: Sized {
    fn new(recipient: Recipient, cart: Vec<Product>) -> Self;
    async fn get(&self, db: &DatabaseConnection, id: &str) -> Option<Self>;
    async fn insert(&self, db: &DatabaseConnection) -> Result<&Self, StoreError>;
}

#[async_trait]
impl Store for Order {
    fn new(recipient: Recipient, cart: Vec<Product>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            date: Utc::now().to_string(),
            status: "CREATED".to_string(),
            recipient,
            cart,
        }
    }
    async fn get(&self, db: &DatabaseConnection, id: &str) -> Option<Self> {
        let order = order::Entity::find_by_id(id.to_string())
            .one(db)
            .await
            .unwrap()
            .unwrap();
        let cart = cart_item::Entity::find()
            .filter(cart_item::Column::OrderId.eq(id))
            .all(db)
            .await
            .unwrap();
        Some(Order {
            id: order.id,
            date: order.date,
            status: order.status,
            recipient: Recipient {
                name: order.recipient_name,
                email: order.recipient_email,
            },
            cart: cart.into_iter().map(|item| item.into()).collect(),
        })
    }

    async fn insert(&self, db: &DatabaseConnection) -> Result<&Self, StoreError> {
        let order_model = order::ActiveModel {
            id: Set(self.id.clone()),
            date: Set(self.date.clone()),
            status: Set(self.status.clone()),
            recipient_name: Set(self.recipient.name.clone()),
            recipient_email: Set(self.recipient.email.clone()),
        };

        order_model.insert(db).await.map_err(|e| {
            error!("Can not save order: {}", e);
            StoreError::SaveError
        })?;

        cart_item::Entity::insert_many(self.cart.iter().map(|item| {
            cart_item::ActiveModel {
                order_id: Set(self.id.clone()),
                product_id: Set(item.id.clone()),
                count: Set(item.count),
                price: Set(item.price),
                title: Set(item.title.clone()),
                ..Default::default()
            }
            .before_save(true).unwrap()
        }))
        .exec(db)
        .await
        .map_err(|e| {
            error!("Can not save cart item: {}", e);
            StoreError::SaveError
        })?;

        Ok(self)
    }
}
