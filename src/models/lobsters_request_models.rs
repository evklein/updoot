use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchTagsResponse {
    tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    id: i32,
    tag: String,
    description: String,
    privileged: bool,
    is_media: bool,
    active: bool,
    hotness_mod: f32,
    permit_by_new_users: bool,
    category_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSubmission {
    short_id: String,
    short_id_url: String,
    created_at: String,
    title: String,
    url: String,
    score: i32,
    flags: i32,
    comment_count: i32,
    description: String,
    description_plain: String,
    comments_url: String,
    pub submitter_user: Lobster,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lobster {
    pub username: String,
    pub created_at: String,
    pub is_admin: bool,
    pub about: String,
    pub is_moderator: bool,
    //pub karma: i32,
    pub avatar_url: String,
    pub invited_by_user: String,
}