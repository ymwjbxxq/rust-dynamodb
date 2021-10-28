pub struct AWSClient {
  config: aws_types::config::Config,
}

impl AWSClient {
   pub fn set_config(config: aws_types::config::Config) -> Self {
    Self { 
      config: config 
    }
  }

  pub fn dynamo_client(&self) -> aws_sdk_dynamodb::Client {
    let dynamo_db_client = aws_sdk_dynamodb::Client::new(&self.config);
    return dynamo_db_client;
  }
}
