use std::sync::Arc;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{
    web::{self},
    App, HttpServer,
};
use ai_voting_agent::api::api::{get_proposals, get_recommendation, get_spaces, post_vote, AppState};
use ai_voting_agent::config::config::Config;
use ai_voting_agent::scheduler::scheduler;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();
    let config = Config::from_env().unwrap();

    let manager =
        PostgresConnectionManager::new_from_stringlike(config.to_pg_connection_string(), NoTls)
            .unwrap();

    let pool = Pool::builder().max_size(10).build(manager).await.unwrap();
    let pool: Arc<Pool<PostgresConnectionManager<NoTls>>> = Arc::new(pool);

    let app_state = AppState { db_client: pool.clone() };

    let sheduler_pool: Arc<Pool<PostgresConnectionManager<NoTls>>> = pool.clone();
   
    tokio::spawn(async move {
        scheduler::start_scheduler(sheduler_pool).await;
    }); 

    println!("start REST API server 0.0.0.0:8080");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .route("/spaces", web::get().to(get_spaces))
            .route("/proposals/{space_id}", web::get().to(get_proposals))
            .route("/recommendation/{proposal_id}", web::get().to(get_recommendation))
            .route("/vote/{proposal_id}", web::post().to(post_vote))
            .route("/get_prop_and_rec", web::get().to(get_prop_and_rec))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}


