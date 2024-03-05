use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::data::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub role: String,
    pub name: Option<String>,
}

// Based on https://platform.openai.com/docs/api-reference/chat/object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatRequestData {
    pub messages: Vec<Message>,

    // Not really necesary but it is part of the OpenAI API. We are going to send the id
    // of the model currently loaded.
    pub model: ModelID,

    pub frequency_penalty: Option<f32>,
    pub logprobs: Option<bool>,
    pub top_logprobs: Option<u32>,
    pub max_tokens: Option<u32>,
    pub presence_penalty: Option<f32>,
    pub seed: Option<u32>,
    pub stop: Option<Vec<String>>,
    pub stream: Option<bool>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,

    // Adding the following fields since there are part of the OpenAI API,
    // but are not likely to be used in the first version of the client
    pub n: Option<u32>,
    pub logit_bias: Option<HashMap<String, f32>>,
}

// Shared structs for ChatResponse and ChatResponseChunk

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageData {
    pub content: String,
    pub role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopLogProbsItemData {
    pub token: String,
    pub logprob: f32,
    pub bytes: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogProbsItemData {
    pub token: String,
    pub logprob: f32,
    pub bytes: Option<Vec<u8>>,
    pub top_logprobs: Vec<TopLogProbsItemData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogProbsData {
    pub content: Vec<LogProbsItemData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StopReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "content_filter")]
    ContentFilter
}

// ChatResponse structs

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChoiceData {
    pub finish_reason: StopReason,
    pub index: u32,
    pub message: MessageData,
    pub logprobs: Option<LogProbsData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsageData {
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatResponseData {
    pub id: String,
    pub choices: Vec<ChoiceData>,
    pub created: u32,
    pub model: ModelID,
    pub system_fingerprint: String,
    pub usage: UsageData,

    #[serde(default = "response_object")]
    pub object: String,
}

fn response_object() -> String {
    "chat.completion".to_string()
}

// ChatResponseChunk structs

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChunkChoiceData {
    pub finish_reason: StopReason,
    pub index: u32,
    pub delta: MessageData,
    pub logprobs: Option<LogProbsData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatResponseChunkData {
    pub id: String,
    pub choices: Vec<ChunkChoiceData>,
    pub created: u32,
    pub model: ModelID,
    pub system_fingerprint: String,

    #[serde(default = "response_chunk_object")]
    pub object: String,
}

fn response_chunk_object() -> String {
    "chat.completion.chunk".to_string()
}

#[derive(Clone, Debug)]
pub enum ChatResponse {
    // https://platform.openai.com/docs/api-reference/chat/object
    ChatFinalResponseData(ChatResponseData),
    // https://platform.openai.com/docs/api-reference/chat/streaming
    ChatResponseChunk(ChatResponseChunkData),
}