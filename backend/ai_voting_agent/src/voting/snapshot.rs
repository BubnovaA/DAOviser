use ethers::prelude::*;
use ethers::signers::Signer;
use ethers::core::types::transaction::eip712::Eip712;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Context};
use chrono::Utc;
use ethers::prelude::*;
use serde_json::json;

// Import the EIP712 trait and helper from eip712_enc
use eip712_enc::{hash_structured_data, EIP712};

use crate::config::config::Config;

/// Helper function to produce a hex string (0x-prefixed) from bytes.
fn hex(s: impl AsRef<[u8]>) -> String {
    format!("0x{}", hex_fmt::HexFmt(s.as_ref()))
}

#[derive(Debug, Clone, Eip712, Serialize, Deserialize)]
#[eip712(
    domain_name = "snapshot",
    domain_version = "0.1.4",
    // For off-chain signing, chain_id can be arbitrary (here using 1)
    chain_id = 1,
    verifying_contract = "0x0000000000000000000000000000000000000000"
)]
pub struct Vote {
    // Primary field to hash for EIP-712 signing.
    #[eip712(primary)]
    pub from: Address,
    pub space: String,
    pub timestamp: u64,
    pub proposal: String,
    pub choice: u32,
    pub reason: String,
    pub app: String,
    pub metadata: String,
}


// Define the structure of the payload to be sent to the Snapshot server.
#[derive(Debug, Serialize)]
struct SnapshotVote {
    address: String,
    msg: Vote,
    sig: String,
}

/// Submits a vote to Snapshot with the given proposal ID and choice.
/// 
/// # Arguments
///
/// * `proposal` - A string representing the proposal ID.
/// * `choice` - A u32 representing the vote option.
/// 
/// # Returns
///
/// * `eyre::Result<()>` indicating success or failure.
pub async fn vote(proposal: String, choice: u32) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    // Replace with your Safe key's private key.
    let private_key = config.safe_wallet_private_key;

    // Create a wallet for signing; chain_id is set to 1 for off-chain signing.
    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(1u64);

    // Capture the current timestamp once.
    let timestamp = Utc::now().timestamp() as u64;

    // Build the EIP‑712 message JSON.
    // Note that we use the same field names and types as required by Snapshot.
    let vote_message = json!({
        "primaryType": "Vote",
        "domain": {
            "name": "snapshot",
            "version": "0.1.4"
        },
        "message": {
            "from": wallet.address().to_string(),
            "space": "arbitrumfoundation.eth",
            "timestamp": timestamp,
            "proposal": proposal,
            "choice": choice,
            "reason": "",
            "app": "snapshot-v2",
            "metadata": ""
        },
        "types": {
            "EIP712Domain": [
                { "name": "name", "type": "string" },
                { "name": "version", "type": "string" }
            ],
            "Vote": [
                { "name": "from", "type": "address" },
                { "name": "space", "type": "string" },
                { "name": "timestamp", "type": "uint64" },
                { "name": "proposal", "type": "string" },
                { "name": "choice", "type": "uint32" },
                { "name": "reason", "type": "string" },
                { "name": "app", "type": "string" },
                { "name": "metadata", "type": "string" }
            ]
        }
    });

    // Convert the JSON into an EIP712 object (from the eip712_enc crate).
    let eip712_data: EIP712 = serde_json::from_value(vote_message)
        .context("Failed to serialize vote message")?;
    // Hash the structured data per EIP‑712.
    let message_hash = hash_structured_data(eip712_data).map_err(|err| anyhow!("{err:?}"))?;
    // Convert the hash (sp_core::H256) to ethers's H256.
    let ethers_hash = ethers::types::H256::from(message_hash.0);

    // Sign the message hash using the wallet.
    let signature = wallet.sign_hash(ethers_hash)?;
    println!("Generated signature: {:?}", signature);

    // Build the final payload to send to Snapshot.
    let payload = json!({
        "address": wallet.address().to_string(),
        "msg": {
            "from": wallet.address().to_string(),
            "space": "arbitrumfoundation.eth",
            "timestamp": timestamp,
            "proposal": proposal,
            "choice": choice,
            "reason": "",
            "app": "snapshot-v2",
            "metadata": ""
        },
        "sig": signature.to_string()
    });

    // Create an asynchronous HTTP client.
    let client = Client::new();
    // Define the Snapshot Hub endpoint (adjust if necessary).
    let snapshot_endpoint = "https://hub.snapshot.org/api/message";

    // Send the signed vote payload as JSON.
    let response = client
        .post(snapshot_endpoint)
        .json(&payload)
        .send()
        .await?;

    // Check for a successful response.
    if response.status().is_success() {
        let resp_text = response.text().await?;
        println!("Vote successfully submitted to Snapshot: {}", resp_text);
    } else {
        eprintln!(
            "Error submitting vote: {}",
            response.text().await.unwrap_or_default()
        );
    }

    Ok(())
}