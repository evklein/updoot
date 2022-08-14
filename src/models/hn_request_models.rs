use futures::poll;
use serde::{Deserialize, Serialize};

pub struct HNMasterStruct {
    pub stories: Vec<Story>,
    pub comments: Vec<Comment>,
    pub asks: Vec<Ask>,
    pub jobs: Vec<Job>,
    pub polls: Vec<Poll>,
}

impl HNMasterStruct {
    pub fn new() -> HNMasterStruct {
        HNMasterStruct {
            stories: Vec::new(),
            comments: Vec::new(),
            asks: Vec::new(),
            jobs: Vec::new(),
            polls: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HNItem {
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    about: String,
    created: i64,
    karma: i32,
    submitted: Vec<i64>,
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Story {
    by: String,
    descendants: i32,
    id: i64,
    kids: Vec<i64>,
    score: i32,
    time: u64,
    title: String,
    #[serde(rename = "type")]
    item_type: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    by: String,
    id: i64,
    kids: Vec<i64>,
    parent: i64,
    text: String,
    time: u64,
    #[serde(rename = "type")]
    item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ask {
    id: i64,
    descendants: i32,
    kids: Vec<i64>,
    score: i32,
    text: String,
    time: i64,
    title: String,
    #[serde(rename = "type")]
    item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    by: String,
    id: i64,
    score: i32,
    text: String,
    time: i64,
    title: String,
    #[serde(rename = "type")]
    item_type: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Poll {
    by: String,
    descendants: i32,
    id: i64,
    kids: Vec<i64>,
    parts: Vec<i64>,
    score: i32,
    text: String,
    time: i64,
    title: String,
    #[serde(rename = "type")]
    item_type: String,
}

// TODO: Poll Part