pub use serde_json::Value;
pub use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Coin{
   pub data: Vec<Value>,
}