use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use serde_json::{json, Value};
use tokio_postgres::types::Json;
use std::error::Error;
use std::sync::Arc;
use tokio_postgres::NoTls;

pub async fn get_recommendation_by_id(
    db_client: &Arc<Pool<PostgresConnectionManager<NoTls>>>,
    id: &String, 
) -> Result<Value, Box<dyn Error>> {
    let conn = db_client.get().await?;
    
    let row = conn
        .query_one(
            r#"
            SELECT 
                proposal_id, 
                technical_impact, 
                economic_consequences, 
                governance_and_decentralization, 
                advantages, 
                risks, 
                recommendation, 
                created_at
            FROM recommendations
            WHERE proposal_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            &[id],
        )
        .await?;
    
    let proposal_id: String = row.get("proposal_id");
    let technical_impact: Value = row.get("technical_impact");
    let economic_consequences: Value = row.get("economic_consequences");
    let governance_and_decentralization: Value = row.get("governance_and_decentralization");
    let advantages: Value = row.get("advantages");
    let risks: Value = row.get("risks");
    let recommendation: Value = row.get("recommendation");
    let created_at: i64 = row.get("created_at");
    
    let result = json!({
        "proposalId": proposal_id,
        "technicalImpact": technical_impact,
        "economicConsequences": economic_consequences,
        "governanceAndDecentralization": governance_and_decentralization,
        "advantages": advantages,
        "risks": risks,
        "recommendation": recommendation,
        "createdAt": created_at,
    });
    
    Ok(result)
}


pub async fn get_new_recommendation(
    db_client: &Arc<Pool<PostgresConnectionManager<NoTls>>>,
    id: &String, 
) -> Result<Value, Box<dyn Error>> {
    let conn = db_client.get().await?;
    
    let row = conn
        .query_one(
            r#"
            SELECT 
                proposal_id, 
                recommendation, 
            FROM recommendations
            WHERE new_flag = true
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            &[id],
        )
        .await?;
    
    let proposal_id: String = row.get("proposal_id");
    let recommendation: Value = row.get("recommendation");
    
    let result = json!({
        "proposalId": proposal_id,
        "voteOption": recommendation,
    });
    
    Ok(result)
}



pub async fn save_recommendation(
    db_client: &Arc<Pool<PostgresConnectionManager<NoTls>>>,
    proposal_id: &String,
    recommendation: &Value,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let conn = db_client.get().await?;
    
    let technical_impact = recommendation.get("technicalImpact").cloned().unwrap_or(Value::Null);
    let economic_consequences = recommendation.get("economicConsequences").cloned().unwrap_or(Value::Null);
    let governance_and_decentralization = recommendation
        .get("governanceAndDecentralization")
        .cloned()
        .unwrap_or(Value::Null);
    let advantages = recommendation.get("advantages").cloned().unwrap_or(Value::Null);
    let risks = recommendation.get("risks").cloned().unwrap_or(Value::Null);
    let rec_value = recommendation.get("recommendation").cloned().unwrap_or(Value::Null);

    let query = r#"
        INSERT INTO recommendations 
            (proposal_id, technical_impact, economic_consequences, governance_and_decentralization, advantages, risks, recommendation)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7)
    "#;

    conn.execute(
        query,
        &[
            proposal_id,
            &Json(technical_impact),
            &Json(economic_consequences),
            &Json(governance_and_decentralization),
            &Json(advantages),
            &Json(risks),
            &Json(rec_value),
        ],
    )
    .await?;

    Ok(())
}
