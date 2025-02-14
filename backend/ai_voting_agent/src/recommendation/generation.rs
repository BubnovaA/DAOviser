use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use log::{error, info};
use std::sync::Arc;
use tokio::time::Duration;
use tokio_postgres::NoTls;

use crate::{
    proposal_snapchot::repository::get_active_proposals_without_rec,
    recommendation::{ai::get_analysis_response, repository::save_recommendation},
};

/// A function that selects active proposals without a recommendation, runs the analyzer 
/// (get_analysis_response), and saves the recommendation.
pub async fn run_recommendation_creator(db_client: &Arc<Pool<PostgresConnectionManager<NoTls>>>) {
    let proposals = get_active_proposals_without_rec(db_client).await;
    if let Ok(proposals) = proposals {
        info!(
            "Found {} proposals without recommendations",
            proposals.len()
        );

        for proposal in proposals {
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            interval.tick().await;
            match get_analysis_response(&proposal.to_string()).await {
                Ok(recommendation) => {
                    let proposal_id = proposal["id"].as_str();
                    if let Some(proposal_id) = proposal_id {
                        match save_recommendation(
                            db_client,
                            &proposal_id.to_string(),
                            &recommendation,
                        )
                        .await {
                            Ok(_) => {
                                info!("Recommendation created for proposal {}", proposal_id);
                            }
                            Err(e) => {
                                error!("Error saving recommendation: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Error analyzing proposal: {}", e);
                }
            }
        }
    } else {
        error!("Error fetching proposals");
    };
    println!("Recommendation creator finished");
}
