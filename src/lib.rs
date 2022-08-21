use std::collections::HashMap;

use models::hn_request_models::HNMasterStruct;
use models::lobsters_request_models::{Lobster, Tag, UserSubmission};
use reqwest::{self, header::USER_AGENT};
use std::{error::Error, thread, time};

pub mod models;
pub mod routes;

const SPOOFED_USER_AGENT_HEADER: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36";

pub struct LobstersClient;

impl LobstersClient {
    pub async fn fetch_tags_async(&self) -> Vec<Tag> {
        let client = reqwest::Client::new();
        let endpoint = String::from("https://lobste.rs/tags.json");
        let raw_response = client
            .get(endpoint)
            .header(USER_AGENT, SPOOFED_USER_AGENT_HEADER)
            .send()
            .await
            .unwrap();

        raw_response.json::<Vec<Tag>>().await.unwrap()
    }

    pub async fn fetch_recent_submissions(&self) -> Vec<UserSubmission> {
        let client = reqwest::Client::new();
        let endpoint = String::from("https://lobste.rs/active.json");
        let raw_response = client
            .get(endpoint)
            .header(USER_AGENT, SPOOFED_USER_AGENT_HEADER)
            .send()
            .await
            .unwrap();

        raw_response.json::<Vec<UserSubmission>>().await.unwrap()
    }

    pub async fn fetch_submissions_on_page(&self, page: i32) -> Vec<UserSubmission> {
        let client = reqwest::Client::new();
        let endpoint = format!("https://lobste.rs/page/{}.json", page);
        let raw_response = client
            .get(&endpoint)
            .header(USER_AGENT, SPOOFED_USER_AGENT_HEADER)
            .send()
            .await
            .unwrap();
        println!("Page {} @ {}", page, endpoint);
        raw_response.json::<Vec<UserSubmission>>().await.unwrap()
    }

    pub async fn fetch_lobster(
        &self,
        username: &str,
    ) -> Result<Lobster, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let endpoint = format!("https://lobste.rs/u/{}.json", username);
        let raw_response = client
            .get(endpoint)
            .header(USER_AGENT, SPOOFED_USER_AGENT_HEADER)
            .send()
            .await?;

        Ok(raw_response.json::<Lobster>().await?)
    }

    pub async fn build_user_tree(&self) -> HashMap<String, Vec<String>> {
        let mut user_tree_map: HashMap<String, Vec<String>> = HashMap::new();

        // Fetch top 4 pages of submissions on lobste.rs
        let recent_submissions: Vec<UserSubmission> = self.fetch_submissions_on_page(0).await;

        let mut users: Vec<Lobster> = Vec::new();
        let mut limit = 0;
        for submission in recent_submissions {
            users.push(submission.submitter_user.clone());
            limit += 1;
            if limit >= 5 {
                break;
            }
        }

        println!("{:?}", users);

        for user in users {
            let mut next_lobster = user;
            loop {
                let (key, invited_by) = (&next_lobster.username, &next_lobster.invited_by_user);
                if user_tree_map.contains_key(invited_by) {
                    let children = user_tree_map.get_mut(invited_by).unwrap();
                    if !children.contains(key) {
                        user_tree_map
                            .get_mut(invited_by)
                            .unwrap()
                            .push(key.to_string());
                    }
                } else {
                    user_tree_map.insert(invited_by.to_string(), vec![key.to_string()]);
                }

                println!("{}->{}", key, invited_by);
                if invited_by == "jcs" {
                    break;
                }
                next_lobster = self.fetch_lobster(invited_by).await.unwrap();
                thread::sleep(time::Duration::from_millis(4000));
            }
            println!("----");
        }

        user_tree_map
    }
}

pub struct HackerNewsClient;

impl HackerNewsClient {
    pub async fn get_latest_item_id(&self) -> Result<i64, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let endpoint = String::from("https://hacker-news.firebaseio.com/v0/maxitem.json");
        let raw_response = client.get(endpoint).send().await?;

        Ok(raw_response.json::<i64>().await.unwrap())
    }

    pub async fn get_latest_items(
        &self,
        number_of_items: i32,
    ) -> Result<HNMasterStruct, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let mut master_list = HNMasterStruct::new();

        let latest_item_id = self.get_latest_item_id().await?;
        let start_index = latest_item_id - number_of_items as i64;

        for next_item_id in start_index..latest_item_id {
            let endpoint = format!(
                "https://hacker-news.firebaseio.com/v0/item/{}.json",
                next_item_id
            );
            let next_item = client.get(endpoint).send().await?.text().await?;
            let next_item_json: serde_json::Value = serde_json::from_str(next_item.as_str())?;

            println!("Item: {}\n{:?}", next_item_id, next_item.as_str());
            match next_item_json["type"].as_str() {
                Some("story") => master_list
                    .stories
                    .push(serde_json::from_str(next_item.as_str())?),
                Some("comment") => master_list
                    .comments
                    .push(serde_json::from_str(next_item.as_str())?),
                Some("ask") => master_list
                    .asks
                    .push(serde_json::from_str(next_item.as_str())?),
                Some("job") => master_list
                    .jobs
                    .push(serde_json::from_str(next_item.as_str())?),
                Some("poll") => master_list
                    .polls
                    .push(serde_json::from_str(next_item.as_str())?),
                _ => panic!("Ahhh!!"),
            };
        }

        Ok(master_list)
    }
}
