use super::{CartItem, Recipient};
use async_graphql::SimpleObject;
use async_trait::async_trait;
use chrono::Utc;
use entity::{cart_item, order};
use futures::future::join_all;
use indoc::formatdoc;
use log::error;
use sea_orm::{
    strum::Display, ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DatabaseConnection,
    EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

#[derive(SimpleObject, Debug)]
pub struct Order {
    pub id: String,
    pub date: String,
    pub status: String,
    pub recipient: Recipient,
    pub cart: Vec<CartItem>,
}

impl Order {
    pub fn new(recipient: Recipient, cart: Vec<CartItem>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            date: Utc::now().to_string(),
            status: "CREATED".to_string(),
            recipient,
            cart,
        }
    }

    pub fn from_db(order: order::Model, cart: Vec<CartItem>) -> Self {
        Self {
            id: order.id,
            date: order.date,
            status: order.status,
            recipient: Recipient {
                name: order.recipient_name,
                email: order.recipient_email,
            },
            cart,
        }
    }

    pub fn to_text(&self) -> String {
        let titles = self
            .cart
            .iter()
            .map(|cart_item| format!("- {}", cart_item.product.title.clone()))
            .collect::<Vec<_>>()
            .join("\n");

        let message = formatdoc! {
            "Новый заказ
            От {name} {email}

            {titles}",

            name = self.recipient.name,
            email = self.recipient.email,
            titles = titles
        };

        message
    }
}

#[derive(Debug, Display)]
pub enum StoreError {
    SaveError,
}

#[async_trait]
pub trait Store: Sized {
    async fn insert(&self, db: &DatabaseConnection) -> Result<&Self, StoreError>;
    async fn get_cart(db: &DatabaseConnection, order_id: &str) -> Vec<CartItem>;
    async fn find(db: &DatabaseConnection) -> Vec<Self>;
    async fn find_one(db: &DatabaseConnection, id: &str) -> Option<Self>;
}

#[async_trait]
impl Store for Order {
    async fn find_one(db: &DatabaseConnection, id: &str) -> Option<Self> {
        let order = order::Entity::find_by_id(id.to_string())
            .one(db)
            .await
            .unwrap()
            .unwrap();
        let cart = Self::get_cart(db, &order.id).await;
        Some(Order::from_db(order, cart))
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
                product_id: Set(item.product.id.clone()),
                count: Set(item.count),
                price: Set(item.price),
                title: Set(item.product.title.clone()),
                ..Default::default()
            }
            .before_save(true)
            .unwrap()
        }))
        .exec(db)
        .await
        .map_err(|e| {
            error!("Can not save cart item: {}", e);
            StoreError::SaveError
        })?;

        Ok(self)
    }

    async fn get_cart(db: &DatabaseConnection, order_id: &str) -> Vec<CartItem> {
        cart_item::Entity::find()
            .filter(cart_item::Column::OrderId.eq(order_id))
            .all(db)
            .await
            .unwrap()
            .into_iter()
            .map(|item| item.into())
            .collect()
    }

    async fn find(db: &DatabaseConnection) -> Vec<Self> {
        let orders = order::Entity::find().all(db).await.unwrap();

        join_all(orders.into_iter().map(|order| async {
            let cart = Self::get_cart(db, &order.id).await;
            Self::from_db(order, cart)
        }))
        .await
    }
}
