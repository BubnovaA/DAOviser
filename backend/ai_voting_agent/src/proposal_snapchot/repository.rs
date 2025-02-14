use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use chrono::{Duration, NaiveDate};
use serde_json::{json, Value};
use std::{error::Error, sync::Arc};
use tokio_postgres::{NoTls, Row};

pub fn row_to_proposal(row: &Row) -> Value {
    json!({
        "id": row.get::<_, String>("id"),
        "ipfs": row.get::<_, Option<String>>("ipfs"),
        "space": row.get::<_, Option<Value>>("space"),
        "type": row.get::<_, Option<String>>("type"),
        "title": row.get::<_, Option<String>>("title"),
        "body": row.get::<_, Option<String>>("body"),
        "discussion": row.get::<_, Option<String>>("discussion"),
        "author": row.get::<_, Option<String>>("author"),
        "quorum": row.get::<_, String>("quorum"),
        "quorum_type": row.get::<_, Option<String>>("quorum_type"),
        "start": row.get::<_, Option<i64>>("start").map(|ts| ts as i64),
        "end": row.get::<_, Option<i64>>("end").map(|ts| ts as i64),
        "snapshot": row.get::<_, Option<String>>("snapshot"),
        "choices": row.get::<_, Option<Value>>("choices"),
        "labels": row.get::<_, Option<Value>>("labels"),
        "scores": row.get::<_, Option<Value>>("scores"),
        "scores_total": row.get::<_, String>("scores_total"),
        "scores_state": row.get::<_, Option<String>>("scores_state"),
        "state": row.get::<_, Option<String>>("state"),
        "strategies": row.get::<_, Option<Value>>("strategies"),
        "created": row.get::<_, Option<i64>>("created").map(|ts| ts as i64),
        "updated": row.get::<_, Option<i64>>("updated").map(|ts| ts as i64),
        "votes": row.get::<_, String>("votes"),
        "privacy": row.get::<_, Option<String>>("privacy"),
        "plugins": row.get::<_, Option<Value>>("plugins"),
        "flagged": row.get::<_, Option<bool>>("flagged"),
    })
}

pub async fn get_proposals_by_date(
    db_client: &Arc<Pool<PostgresConnectionManager<NoTls>>>,
    date_str: &str,
) -> Result<Value, Box<dyn Error>> {
    let conn = db_client.get().await?;
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    let start_dt = date
        .and_hms_opt(0, 0, 0)
        .ok_or("Invalid start date")?;
    let start_unixtime = start_dt.and_utc().timestamp() as f64;
    let end_unixtime = (start_dt + Duration::days(1)).and_utc().timestamp() as f64;

    let rows = conn
        .query(
            "SELECT 
                id, ipfs, space, \"type\", title, body, discussion, author, quorum, quorum_type,
                (EXTRACT(EPOCH FROM \"start\"))::int8 as start, (EXTRACT(EPOCH FROM \"end\"))::int8 as end, snapshot, choices, labels, scores, scores_total, scores_state, state,
                strategies, (EXTRACT(EPOCH FROM created))::int8 as created, (EXTRACT(EPOCH FROM updated))::int8 as updated, votes, privacy, plugins, flagged
             FROM proposals
             WHERE \"start\" >= TO_TIMESTAMP($1) AND \"start\" < TO_TIMESTAMP($2)
             ORDER BY \"start\" DESC",
            &[&start_unixtime, &end_unixtime],
        )
        .await?;

    let proposals: Vec<Value> = rows.into_iter().map(|row| row_to_proposal(&row)).collect();

    Ok(json!(proposals))
}

pub async fn get_proposals_by_space_id(
    db_client: &Arc<Pool<PostgresConnectionManager<NoTls>>>,
    space_id: &String,
) -> Result<Value, Box<dyn Error>> {
    let conn = db_client.get().await?;

    let rows = conn
    .query(
        "SELECT 
            id, ipfs, space, \"type\", title, body, discussion, author, quorum, quorum_type,
            (EXTRACT(EPOCH FROM \"start\"))::int8 as start, (EXTRACT(EPOCH FROM \"end\"))::int8 as end, snapshot, choices, labels, scores, scores_total, scores_state, state,
            strategies, (EXTRACT(EPOCH FROM created))::int8 as created, (EXTRACT(EPOCH FROM updated))::int8 as updated, votes, privacy, plugins, flagged
         FROM proposals
         WHERE space->>'id' = $1 AND state = 'active' AND \"end\" >= NOW()
         ORDER BY \"created\" DESC",
        &[&space_id],
    )
    .await?;

    let proposals: Vec<Value> = rows.into_iter().map(|row| row_to_proposal(&row)).collect();

    Ok(json!(proposals))
}

pub async fn get_proposals_by_id(
    db_client: &Arc<Pool<PostgresConnectionManager<NoTls>>>,
    id: &String,
) -> Result<Value, Box<dyn Error>> {
    let conn = db_client.get().await?;
    let rows = conn
        .query(
            "SELECT 
                id, ipfs, space, \"type\", title, body, discussion, author, quorum, quorum_type,
                (EXTRACT(EPOCH FROM \"start\"))::int8 as start, (EXTRACT(EPOCH FROM \"end\"))::int8 as end, snapshot, choices, labels, scores, scores_total, scores_state, state,
                strategies, (EXTRACT(EPOCH FROM created))::int8 as created, (EXTRACT(EPOCH FROM updated))::int8 as updated, votes, privacy, plugins, flagged
             FROM proposals
             WHERE id = $1
             ORDER BY \"start\" DESC",
            &[&id],
        )
        .await?;

    let proposals: Vec<Value> = rows.into_iter().map(|row| row_to_proposal(&row)).collect();

    Ok(json!(proposals))
}

pub async fn get_active_proposals_without_rec(
    db_client: &Arc<Pool<PostgresConnectionManager<NoTls>>>,
) -> Result<Vec<Value>, Box<dyn Error + Send + Sync>> {
    let conn = db_client.get().await?;
    let query = r#"
    SELECT 
        id, ipfs, space, "type", title, body, discussion, author, quorum, quorum_type,
        (EXTRACT(EPOCH FROM "start"))::int8 as start, (EXTRACT(EPOCH FROM "end"))::int8 as end, snapshot, choices, labels, scores, scores_total, scores_state, state,
        strategies, (EXTRACT(EPOCH FROM created))::int8 as created, (EXTRACT(EPOCH FROM updated))::int8 as updated, votes, privacy, plugins, flagged
    FROM proposals
    WHERE state = 'active'
      AND "end" >= NOW()
      AND id NOT IN (SELECT proposal_id FROM recommendations)
    "#;
    // AND "end" <= NOW() + INTERVAL '5 days'
    println!("query: {}", query);
    let rows = conn.query(query, &[]).await?;

    println!("Found {} proposals without recommendations", rows.len());

    let proposals: Vec<Value> = rows.into_iter().map(|row| row_to_proposal(&row)).collect();

    Ok(proposals)
}

pub async fn get_spaces_vec(
    db_client: &Arc<Pool<PostgresConnectionManager<NoTls>>>,
) -> Result<Value, Box<dyn Error>> {
    let conn = db_client.get().await?;

    let rows = conn
        .query(
            "SELECT 
                space->>'id' AS space_id, 
                space->>'name' AS space_name, 
                space->>'avatar' AS space_avatar, 
                COUNT(*) FILTER (WHERE state = 'active' AND \"end\" >= NOW()) AS active_proposals_count,
                COUNT(*) AS proposals_count
            FROM public.proposals
            GROUP BY space->>'id', space->>'name', space->>'avatar'
            HAVING COUNT(*) FILTER (WHERE state = 'active' AND \"end\" >= NOW()) > 0
            ORDER BY proposals_count DESC
            LIMIT 50",
            &[],
        )
        .await?;

    let spaces: Vec<Value> = rows.into_iter().map(|row| row_to_space(&row)).collect();

    Ok(json!(spaces))
}

fn row_to_space(row: &Row) -> Value {
    json!({
        "space_id": row.get::<_, Option<String>>(0).unwrap_or_default(),
        "space_name": row.get::<_, Option<String>>(1).unwrap_or_default(),
        "space_avatar": row.get::<_, Option<String>>(2).unwrap_or_default(),
        "active_proposals_count": row.get::<_, Option<i64>>(3).unwrap_or(0),
        "proposals_count": row.get::<_, Option<i64>>(4).unwrap_or(0),
    })
}
