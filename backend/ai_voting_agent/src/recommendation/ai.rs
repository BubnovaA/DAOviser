use rig::{
    completion::{CompletionModel, CompletionRequest, ModelChoice},
    providers::openai,
};
use serde_json::Value;
use std::error::Error;

pub async fn get_analysis_response(proposal_json: &String) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Initialize the OpenAI client using environment variables
    let client = openai::Client::from_env();

    // Create a model instance
    let model = client.completion_model(openai::GPT_35_TURBO);

    let system_message = "You act as an expert with deep knowledge in blockchain technologies, decentralized organizations, and DAO management, particularly regarding the Arbitrum DAO.";

    let user_prompt = format!("Please help analyze the proposal in the Arbitrum DAO. Here is its brief description: [{}].\n\n\
        Analyze the proposal from the following perspectives:
        1. Technical impact on protocol development.
        2. Economic consequences for ecosystem sustainability.
        3. Governance and decentralization aspects for the community.

        For each of these points, provide the following data in JSON format with key-value pairs:
        - technicalImpact: A detailed description of the technical impact.
        - economicConsequences: An analysis of the economic consequences for the ecosystem.
        - governanceAndDecentralization: An evaluation of the governance and decentralization aspects.
        - advantages: A list of the key advantages of the proposal.
        - risks: A list of the key risks and potential negative aspects.
        - recommendation: An object with suggested voting options and their weights (the sum of weights must equal 1), for example:
        \"recommendation\": {{ \"For\": 0.8, \"Against\": 0.2 }}", proposal_json );

    
        let request = CompletionRequest {
            preamble: Some(system_message.to_string()),
            chat_history: Vec::new(),
            prompt: user_prompt,
            temperature: Some(0.3),
            additional_params: None,
            tools: Vec::new(),
            documents: Vec::new(),
            max_tokens: Some(256),
        };
  

    let response = model.completion(request).await?;

    let answer_str = match response.choice {
        ModelChoice::Message(text) => text,
        ModelChoice::ToolCall(_, _placeholder, args) => {
            args.to_string()
        }
    };

    let cleaned_answer = clean_markdown(&answer_str);

    let json_response: Value = serde_json::from_str(&cleaned_answer)?;

    Ok(json_response)
}


fn clean_markdown(text: &str) -> String {
    let trimmed = text.trim();
    let without_start = if trimmed.starts_with("```") {
        let mut lines = trimmed.lines();
        lines.next();
        lines.collect::<Vec<_>>().join("\n")
    } else {
        trimmed.to_string()
    };
    let without_end = if without_start.trim_end().ends_with("```") {
        let mut lines = without_start.lines().collect::<Vec<_>>();
        if let Some(last) = lines.last() {
            if last.trim() == "```" {
                lines.pop();
            }
        }
        lines.join("\n")
    } else {
        without_start
    };
    without_end.trim().to_string()
}
