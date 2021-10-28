pub mod handler {
  use crate::dtos::request::Request;
  use crate::models::product::Product;
  use crate::queries::get_by_id::GetById;
  use crate::queries::get_by_id::GetByIdQuery;
  use lambda_runtime::{Context, Error};
  use serde_json::Value;

  pub async fn execute(client: &aws_sdk_dynamodb::Client, event: Value, _ctx: Context) -> Result<Option<Product>, Error> {
    log::info!("input {:?}", event);
    let request: Request = serde_json::from_value(event)?;
    let id = request.pk.expect("id must be set");

    log::info!("Fetching product {}", id);

    let product = GetById::new()
                            .await.execute(&client, &id)
                            .await?;

    Ok(match product {
      Some(item) => Some(item),
      None => None,
    })
  }
}
