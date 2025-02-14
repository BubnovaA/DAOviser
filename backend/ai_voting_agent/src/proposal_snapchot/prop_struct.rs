use serde_json::Value;

#[derive(Debug)]
pub struct Proposal {
    pub id: String,
    pub ipfs: Option<String>,
    pub space: Option<Value>,
    pub proposal_type: Option<String>, 
    pub title: Option<String>,
    pub body: Option<String>,
    pub discussion: Option<String>,
    pub author: Option<String>,
    pub quorum: String,
    pub quorum_type: Option<String>,
    pub start: Option<i64>, // Unix‑time
    pub end: Option<i64>,   // Unix‑time
    pub snapshot: Option<String>,
    pub choices: Option<Value>,
    pub labels: Option<Value>,
    pub scores: Option<Value>,
    pub scores_total: String,
    pub scores_state: Option<String>,
    pub state: Option<String>,
    pub strategies: Option<Value>,
    pub created: Option<i64>, // Unix‑time
    pub updated: Option<i64>, // Unix‑time
    pub votes: String,
    pub privacy: Option<String>,
    pub plugins: Option<Value>,
    pub flagged: Option<bool>,
}

impl Proposal {
    pub fn from_json(p: &Value) -> Option<Self> {
        let id = p.get("id")?.as_str()?.to_string();
        let ipfs = p.get("ipfs").and_then(|v| v.as_str().map(|s| s.to_string()));
        let space = p.get("space").cloned();
        let proposal_type = p.get("type").and_then(|v| v.as_str().map(|s| s.to_string()));
        let title = p.get("title").and_then(|v| v.as_str().map(|s| s.to_string()));
        let body = p.get("body").and_then(|v| v.as_str().map(|s| s.to_string()));
        let discussion = p.get("discussion").and_then(|v| v.as_str().map(|s| s.to_string()));
        let author = p.get("author").and_then(|v| v.as_str().map(|s| s.to_string()));
        let quorum = match p.get("quorum") {
            Some(v) => {
                if let Some(n) = v.as_f64() {
                    n.to_string()
                } else if let Some(s) = v.as_str() {
                    s.to_string()
                } else {
                    "0".to_string()
                }
            },
            None => "0".to_string(),
        };
        let quorum_type = p.get("quorumType").and_then(|v| v.as_str().map(|s| s.to_string()));
        let start = p.get("start").and_then(|v| v.as_i64());
        let end = p.get("end").and_then(|v| v.as_i64());
        let snapshot = p.get("snapshot").and_then(|v| v.as_str().map(|s| s.to_string()));
        let choices = p.get("choices").cloned();
        let labels = p.get("labels").cloned();
        let scores = p.get("scores").cloned();
        let scores_total = match p.get("scores_total") {
            Some(v) => {
                if let Some(n) = v.as_f64() {
                    n.to_string()
                } else if let Some(s) = v.as_str() {
                    s.to_string()
                } else {
                    "0".to_string()
                }
            },
            None => "0".to_string(),
        };
        let scores_state = p.get("scores_state").and_then(|v| v.as_str().map(|s| s.to_string()));
        let state = p.get("state").and_then(|v| v.as_str().map(|s| s.to_string()));
        let strategies = p.get("strategies").cloned();
        let created = p.get("created").and_then(|v| v.as_i64());
        let updated = p.get("updated").and_then(|v| v.as_i64());
        let votes = match p.get("votes") {
            Some(v) => {
                if let Some(n) = v.as_f64() {
                    n.to_string()
                } else if let Some(s) = v.as_str() {
                    s.to_string()
                } else {
                    "0".to_string()
                }
            },
            None => "0".to_string(),
        };
        let privacy = p.get("privacy").and_then(|v| v.as_str().map(|s| s.to_string()));
        let plugins = p.get("plugins").cloned();
        let flagged = p.get("flagged").and_then(|v| v.as_bool());

        Some(Proposal {
            id,
            ipfs,
            space,
            proposal_type,
            title,
            body,
            discussion,
            author,
            quorum,
            quorum_type,
            start,
            end,
            snapshot,
            choices,
            labels,
            scores,
            scores_total,
            scores_state,
            state,
            strategies,
            created,
            updated,
            votes,
            privacy,
            plugins,
            flagged,
        })
    }
}



