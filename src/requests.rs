use reqwest::Client;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, FromRepr};

#[derive(FromRepr, EnumIter, Display)]
pub enum RequestType {
    Add,
    Get,
    GetForTable,
    GetAll,
    Remove,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Add {
    pub name: String,
    pub table_id: i32,
    pub quantity: i32,
}

pub async fn add_one(
    worker_id: i32,
    name: &str,
    table_id: i32,
    quantity: i32,
) -> Result<(), reqwest::Error> {
    let url: String = format!("http://localhost:8080/item");
    let executor = Client::new();
    let req = Add {
        name: name.to_string(),
        table_id,
        quantity,
    };
    let resp = executor.post(&url).json(&req).send().await?;
    let body = &resp.text().await?;
    println!(
        "Worker {} finished task add_one for table {}. Body: {}",
        worker_id, table_id, body
    );
    Ok(())
}

pub async fn get_item(worker_id: i32, item_id: i32) -> Result<(), reqwest::Error> {
    let url: String = format!("http://localhost:8080/item/{}", item_id);
    let executor = Client::new();
    let resp = executor.get(&url).send().await?;
    let body = &resp.text().await?;
    println!(
        "Worker {} finished task get_item for item {}. Body: {}",
        worker_id, item_id, body
    );
    Ok(())
}

pub async fn get_items_for_table(worker_id: i32, table_id: i32) -> Result<(), reqwest::Error> {
    let url: String = format!("http://localhost:8080/table/{}", table_id);
    let executor = Client::new();
    let resp = executor.get(&url).send().await?;
    let body = &resp.text().await?;
    println!(
        "Worker {} finished task get_items_for_table for table {}. Body: {}",
        worker_id, table_id, body
    );
    Ok(())
}

pub async fn get_all_items(worker_id: i32) -> Result<(), reqwest::Error> {
    let url: String = "http://localhost:8080/items".to_string();
    let executor = Client::new();
    let resp = executor.get(&url).send().await?;
    let body = &resp.text().await?;
    println!(
        "Worker {} finished task get_all_items. Body: {}",
        worker_id, body
    );
    Ok(())
}

pub async fn remove_item(
    worker_id: i32,
    item_id: i32,
    quantity: i32,
) -> Result<(), reqwest::Error> {
    let url: String = format!("http://localhost:8080/item/{}/{}", item_id, quantity);
    let executor = Client::new();
    let resp = executor.delete(&url).send().await?;
    let body = &resp.text().await?;
    println!(
        "Worker {} finished task remove_item for item {}. Body: {}",
        worker_id, item_id, body
    );
    Ok(())
}
