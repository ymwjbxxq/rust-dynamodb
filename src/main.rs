use lambda_runtime::{handler_fn, Error};
use log::LevelFilter;
use simple_logger::SimpleLogger;

mod library;
mod dtos;
mod models;
mod queries;
mod errors;
use library::lambda::handler::execute;

#[tokio::main]
async fn main() -> Result<(), Error> {
  // required to enable CloudWatch error logging by the runtime
  SimpleLogger::new()
    .with_level(LevelFilter::Info)
    .init()
    .unwrap();

  let func = handler_fn(execute);
  lambda_runtime::run(func).await?;
  Ok(())
}