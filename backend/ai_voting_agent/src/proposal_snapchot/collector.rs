use crate::proposal_snapchot::prop_struct::Proposal;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use chrono::DateTime;
use log::{error, info};
use reqwest::Client as HttpClient;
use serde_json::{json, Value};
use std::{collections::HashMap, error::Error, sync::Arc};
use tokio_postgres::NoTls;

/// URL GraphQL
const GRAPHQL_URL: &str = "https://hub.snapshot.org/graphql";

const BATCH_SIZE: i64 = 1000;

const PROPOSALS_QUERY_TEMPLATE: &str = r#"
query Proposals($first: Int!, $start: Int!) {
  proposals(
    first: $first
     where: { replace_key: $start }
    orderBy: "created"
    orderDirection: asc
  ) {
    ...offchainProposalFragment
    start
  }
}

fragment offchainProposalFragment on Proposal {
  id
  ipfs
  space {
    id
    name
    avatar
    network
    admins
    moderators
    symbol
    terms
  }
  type
  title
  body
  discussion
  author
  quorum
  quorumType
  start
  end
  snapshot
  choices
  labels
  scores
  scores_total
  scores_state
  state
  strategies {
    name
    params
    network
  }
  created
  updated
  votes
  privacy
  plugins
  flagged
}

"#;

pub fn proposals_query_create() -> String {
    PROPOSALS_QUERY_TEMPLATE.replace("replace_key", "created_gt")
}

pub fn proposals_query_update() -> String {
    PROPOSALS_QUERY_TEMPLATE.replace("replace_key", "updated_gt")
}

pub fn proposals_query(key: &str) -> String {
    match key {
        "created" => proposals_query_create(),
        "updated" => proposals_query_update(),
        _ => panic!("Unknown key: {}", key),
    }
}

pub async fn get_proposals(
    max_value: i64,
    key: &str,
) -> Result<Vec<Proposal>, Box<dyn Error + Send + Sync>> {
    let http_client = HttpClient::new();
    let mut new_proposals: Vec<Proposal> = Vec::new();
    let mut last_start = max_value;
    loop {
        let query = proposals_query(key);

        let variables = json!({
            "first": BATCH_SIZE,
            "start": last_start
        });
        let body = json!({
            "query": query,
            "variables": variables
        });
        let response = http_client.post(GRAPHQL_URL).json(&body).send().await?;
        
        if !response.status().is_success() {
            error!(
                "GraphQL query for new proposals failed with status: {}",
                response.status()
            );
            break;
        }
        let resp_json: Value = response.json().await?;
        if resp_json.get("errors").is_some() {
            error!(
                "GraphQL errors for new proposals: {:?}",
                resp_json.get("errors")
            );
            break;
        }
        let proposals_array = resp_json
            .get("data")
            .and_then(|data| data.get("proposals"))
            .and_then(|arr| arr.as_array())
            .ok_or("No proposals array in new proposals response")?;
        if proposals_array.is_empty() {
            info!("No new proposals found; ending collection of new proposals.");
            break;
        }
        for p in proposals_array {
            if let Some(proposal) = Proposal::from_json(p) {

                new_proposals.push(proposal);
            }
        }
        if let Some(last) = proposals_array.last() {
            if let Some(ts) = last.get(key).and_then(|v| v.as_i64()) {
                last_start = ts;
            }
        }
        if proposals_array.len() < BATCH_SIZE as usize {
            break;
        }
    }
    info!("Found {} proposals", new_proposals.len());

    Ok(new_proposals)
}

pub async fn collect_new_proposals(
    db_client: &Arc<Pool<PostgresConnectionManager<NoTls>>>,
    key: &str,
) -> Result<Vec<Proposal>, Box<dyn Error + Send + Sync>> {
    let db_client = db_client.get().await?;
    let query = format!("SELECT COALESCE(MAX(EXTRACT(EPOCH FROM \"{}\"))::INT8, 0) AS max_value FROM proposals", key);
    // Извлекаем максимальное значение "start" из базы
    let row = db_client
        .query_one(
            query.as_str(),
            &[],
        )
        .await?;
    let max_value: i64 = row.get("max_value");


    let new_proposals = get_proposals(max_value, key).await?;
    info!("Collecting new proposals: start from = {}", max_value);

    Ok(new_proposals)
}


pub async fn run_collect(
    db_client: &Arc<Pool<PostgresConnectionManager<NoTls>>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let proposals = collect_proposals(db_client).await?;
    let _ = upsert_proposals(db_client, &proposals).await?;
    Ok(())
}


pub async fn collect_proposals(
    db_client: &Arc<Pool<PostgresConnectionManager<NoTls>>>,
) -> Result<Vec<Proposal>, Box<dyn Error + Send + Sync>> {
    let new_props = collect_new_proposals(db_client,  "created").await?;
    let updated_props = collect_new_proposals(db_client,  "updated").await?;
    
    let mut proposals_map: HashMap<String, Proposal> = HashMap::new();
    for proposal in new_props.into_iter().chain(updated_props.into_iter()) {
        proposals_map.insert(proposal.id.clone(), proposal);
    }
    let proposals_to_upsert: Vec<Proposal> = proposals_map
        .into_iter()
        .map(|(_id, proposal)| proposal)
        .collect();
    info!("Total proposals to upsert: {}", proposals_to_upsert.len());
    Ok(proposals_to_upsert)
}

pub async fn upsert_proposals(
    db_client: &Arc<Pool<PostgresConnectionManager<NoTls>>>,
    proposals: &[Proposal],
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if proposals.is_empty() {
        info!("No proposals to upsert.");
        return Ok(());
    }
    let mut db_client = db_client.get().await?;
    let transaction = db_client.transaction().await?;

    for proposal in proposals.iter() {
        let updated  = proposal.updated.and_then(|ts| DateTime::from_timestamp(ts, 0));
        let start  = proposal.start.and_then(|ts| DateTime::from_timestamp(ts, 0));
        let end = proposal.end.and_then(|ts| DateTime::from_timestamp(ts, 0));
        let created  = proposal.created.and_then(|ts| DateTime::from_timestamp(ts, 0));
        transaction.execute(
            "INSERT INTO proposals (
                id, ipfs, space, \"type\", title, body, discussion, author, quorum, quorum_type,
                \"start\", \"end\", snapshot, choices, labels, scores, scores_total, scores_state, state,
                strategies, created, updated, votes, privacy, plugins, flagged
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17, $18, $19,
                $20, $21, $22, $23, $24, $25, $26
            )
            ON CONFLICT (id) DO UPDATE SET
                space = EXCLUDED.space,
                state = EXCLUDED.state,
                discussion = EXCLUDED.discussion,
                scores = EXCLUDED.scores,
                scores_total = EXCLUDED.scores_total, 
                scores_state = EXCLUDED.scores_state, 
                votes = EXCLUDED.votes,
                updated = EXCLUDED.updated",
            &[
                &proposal.id,
                &proposal.ipfs,
                &proposal.space,
                &proposal.proposal_type,
                &proposal.title,
                &proposal.body,
                &proposal.discussion,
                &proposal.author,
                &proposal.quorum,
                &proposal.quorum_type,
                &start.map(|dt| dt.naive_utc()),
                &end.map(|dt| dt.naive_utc()),
                &proposal.snapshot,
                &proposal.choices,
                &proposal.labels,
                &proposal.scores,
                &proposal.scores_total,
                &proposal.scores_state,
                &proposal.state,
                &proposal.strategies,
                &created.map(|dt| dt.naive_utc()),
                &updated.map(|dt| dt.naive_utc()),
                &proposal.votes,
                &proposal.privacy,
                &proposal.plugins,
                &proposal.flagged,
            ]
        ).await?;
    }
    transaction.commit().await?;
    Ok(())
}

