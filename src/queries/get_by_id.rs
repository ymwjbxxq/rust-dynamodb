use crate::errors::error::Error;
use crate::models::product::Product;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use async_trait::async_trait;

#[async_trait]
pub trait GetByIdQuery {
    async fn new(client: Client) -> Self;
    async fn execute(&self, pk: &str) -> Result<Option<Product>, Error>;
}

#[derive(Debug)]
pub struct GetById {
  client: Client,
  table_name: String,
}

#[async_trait]
impl GetByIdQuery for GetById {
  async fn new(client: Client) -> Self {
    let table_name = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    Self { client, table_name }
  }

  async fn execute(&self, pk: &str) -> Result<Option<Product>, Error> {
    let res = self
      .client
      .get_item()
      .table_name(&self.table_name)
      .key("pk", AttributeValue::S(pk.to_owned()))
      .send()
      .await?;

    Ok(match res.item {
      None => None,
      Some(item) => Some(Product::from_dynamodb(item)?),
    })
  }
}
