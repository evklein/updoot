use serde::{Deserialize, Serialize};

#[derive(Debug)]
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
pub struct User {
    #[serde(default)]
    about: String,

    #[serde(default)]
    created: i64,

    #[serde(default)]
    karma: i32,

    #[serde(default)]
    submitted: Vec<i64>,

    #[serde(default)]
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Story {
    #[serde(default)]
    by: String,

    #[serde(default)]
    descendants: i32,

    #[serde(default)]
    id: i64,

    #[serde(default)]
    kids: Vec<i64>,

    #[serde(default)]
    score: i32,
    #[serde(default)]
    time: u64,

    #[serde(default)]
    title: String,

    #[serde(rename = "type")]
    item_type: String,

    #[serde(default)]
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Comment {
    #[serde(default)]
    pub by: String,

    #[serde(default)]
    pub id: i64,

    #[serde(default)]
    pub kids: Vec<i64>,

    #[serde(default)]
    pub parent: i64,

    #[serde(default)]
    pub text: String,

    #[serde(default)]
    pub time: u64,

    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ask {
    #[serde(default)]
    id: i64,

    #[serde(default)]
    descendants: i32,

    #[serde(default)]
    kids: Vec<i64>,

    #[serde(default)]
    score: i32,

    #[serde(default)]
    text: String,

    #[serde(default)]
    time: i64,

    #[serde(default)]
    title: String,

    #[serde(rename = "type")]
    item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    #[serde(default)]
    by: String,

    #[serde(default)]
    id: i64,

    #[serde(default)]
    score: i32,

    #[serde(default)]
    text: String,

    #[serde(default)]
    time: i64,

    #[serde(default)]
    title: String,

    #[serde(rename = "type")]
    item_type: String,

    #[serde(default)]
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Poll {
    #[serde(default)]
    by: String,

    #[serde(default)]
    descendants: i32,

    #[serde(default)]
    id: i64,

    #[serde(default)]
    kids: Vec<i64>,

    #[serde(default)]
    parts: Vec<i64>,

    #[serde(default)]
    score: i32,

    #[serde(default)]
    text: String,

    #[serde(default)]
    time: i64,

    #[serde(default)]
    title: String,

    #[serde(rename = "type")]
    item_type: String,
}

// TODO: Poll Part
