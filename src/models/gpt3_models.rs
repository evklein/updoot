use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GPT3RequestModel {
    pub model: String,
    pub prompt: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GPT3ResponseModel {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<GPT3Choice>,
    pub usage: GPT3Usage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GPT3Choice {
    pub text: String,
    pub index: u32,
    pub logprobs: Option<usize>,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GPT3Usage {
    pub prompt_tokens: usize,
    pub completions_tokens: usize,
    pub total_tokens: usize,
}