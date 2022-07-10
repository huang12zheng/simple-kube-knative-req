use kube::{
    api::{Api, PostParams},
    core::{DynamicObject, GroupVersionKind},
    discovery, Client,
};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    Ok(())
}
