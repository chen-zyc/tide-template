mod config;
mod log;
mod prelude;
mod routes;
mod web;

use prelude::*;

#[async_std::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    print_all_environment();
    let _work_guard = log::init();

    web::init().await
}

#[allow(dead_code)]
fn print_all_environment() {
    debug!("ALL ENVIRONMENT:");
    for (key, value) in std::env::vars() {
        debug!("\t{:20}: {}", key, value);
    }
}
