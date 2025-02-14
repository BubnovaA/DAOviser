use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use log::{error, info};
use serde::Deserialize;
use tokio_postgres::NoTls;

use crate::{
    proposal_snapchot::repository::{
        get_proposals_by_id, get_proposals_by_space_id, get_spaces_vec,
    },
    recommendation::{
        ai::get_analysis_response,
        repository::{get_recommendation_by_id, save_recommendation, get_new_recommendation},
    },
};

#[derive(Deserialize)]
pub struct ProposalsQueryParams {
    pub date: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db_client: Arc<Pool<PostgresConnectionManager<NoTls>>>,
}

pub async fn get_proposals(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let space_id = path.into_inner();
    match get_proposals_by_space_id(&app_state.db_client, &space_id).await {
        Ok(json_value) => HttpResponse::Ok().json(json_value),
        Err(err) => {
            eprintln!("Error fetching proposals: {}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn get_spaces(app_state: web::Data<AppState>) -> impl Responder {
    match get_spaces_vec(&app_state.db_client).await {
        Ok(json_value) => HttpResponse::Ok().json(json_value),
        Err(err) => {
            eprintln!("Error fetching spaces: {}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn get_recommendation(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let proposal_id = path.into_inner();
    info!("proposal_id: {:?}", proposal_id);
    let recommendation = get_recommendation_by_id(&app_state.db_client, &proposal_id).await;
    match recommendation {
        Ok(json_value) => HttpResponse::Ok().json(json_value),
        Err(err) => {
            eprintln!("Error fetching recommendation: {}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}


pub async fn get_prop_and_rec(
    app_state: web::Data<AppState>,
) -> impl Responder {
    let recommendation = get_new_recommendation(&app_state.db_client).await;
    match recommendation {
        Ok(json_value) => HttpResponse::Ok().json(json_value),
        Err(err) => {
            eprintln!("Error fetching recommendation: {}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn update_recommendation(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let proposal_id = path.into_inner();
    let proposal_json = get_proposals_by_id(&app_state.db_client, &proposal_id).await;

    if let Ok(proposal) = proposal_json {
        println!("proposal_json: {:?}", proposal);
        let recommendation = get_analysis_response(&proposal.to_string()).await;
        match recommendation {
            Ok(recommendation) => {
                let _ =
                    save_recommendation(&app_state.db_client, &proposal_id, &recommendation).await;
                HttpResponse::Ok().body(format!("Recommendation: {}", recommendation))
            }
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    } else {
        let err = "Proposal not found";
        HttpResponse::InternalServerError().body(err.to_string())
    }
}

pub async fn post_vote(path: web::Path<String>) -> impl Responder {
    let proposal_id = path.into_inner();

    HttpResponse::Ok().body(format!("Voting for proposal: {}", proposal_id))
}
