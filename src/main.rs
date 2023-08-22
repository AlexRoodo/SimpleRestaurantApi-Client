use generators::random_numb;
use tokio::runtime::Runtime;

mod generators;
mod random;
mod requests;

const STAFF_SIZE: usize = 15;
const TASK_AMOUNT: i32 = 150;
const TABLES_AMOUNT: i32 = 200;

#[tokio::main]
async fn main() {
    let pool = threadpool::ThreadPool::with_name("tablets pool".to_string(), STAFF_SIZE);

    for i in 1..=TASK_AMOUNT {
        pool.execute(move || {
            let runtime = Runtime::new().unwrap();
            let max_table_id = random_numb(10, TABLES_AMOUNT);
            let min_table_id = max_table_id - 10;

            match runtime
                .block_on(async { random::call_random_action(i, min_table_id, max_table_id).await })
            {
                Ok(_) => {}
                Err(e) => println!("Error : {}", e),
            }
        });
    }
    pool.join();
}
