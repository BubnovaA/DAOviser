use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use log::info;
use std::sync::Arc;
use tokio::time::Duration;
use tokio_postgres::NoTls;

use crate::{
    proposal_snapchot::collector::run_collect,
    recommendation::generation::run_recommendation_creator,
};

pub async fn start_scheduler(pool: Arc<Pool<PostgresConnectionManager<NoTls>>>) {
    let mut interval = tokio::time::interval(Duration::from_secs(300));
    loop {
        {
            info!("Scheduler: collecting proposals");
            let res = run_collect(&pool).await;
            match res {
                Ok(_) => {
                    println!("Scheduler: collecting proposals finished");
                }
                Err(e) => {
                    println!("Scheduler: collecting proposals error: {}", e);
                }
            }
        }
        println!("Scheduler: generating recommendations ");
        {
            let _ = run_recommendation_creator(&pool).await;
        }
        interval.tick().await;
    }
}
