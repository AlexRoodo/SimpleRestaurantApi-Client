use rand::Rng;
use strum::IntoEnumIterator;

use crate::{
    generators::{self},
    requests::{
        add_one, get_all_items, get_item, get_items_for_table, remove_item,
        RequestType::{self},
    },
};

fn get_random_action() -> RequestType {
    let mut rnd = rand::thread_rng();
    let limit = RequestType::iter().count();
    let idx = rnd.gen_range(0..limit);
    RequestType::from_repr(idx).unwrap_or(RequestType::Add)
}

pub async fn call_random_action(
    worker_id: i32,
    min_table_id: i32,
    max_table_id: i32,
) -> Result<(), reqwest::Error> {
    let action = get_random_action();
    let name = generators::random_string();
    let item_id = generators::random_numb(1, 200);
    let table_id = generators::random_numb(min_table_id, max_table_id);
    let quantity = generators::random_numb(1, 10);

    println!(
        "Worker {} with assinged tables {} to {} starting action: {}",
        worker_id, min_table_id, max_table_id, action
    );

    match action {
        RequestType::Add => add_one(worker_id, name.as_str(), table_id, quantity).await,
        RequestType::Get => get_item(worker_id, item_id).await,
        RequestType::GetForTable => get_items_for_table(worker_id, table_id).await,
        RequestType::GetAll => get_all_items(worker_id).await,
        RequestType::Remove => remove_item(worker_id, item_id, quantity).await,
    }
}
