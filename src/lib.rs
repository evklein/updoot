use std::{collections::HashMap};

use std::{thread, time};
use models::lobsters_request_models::{UserSubmission, Tag, Lobster};
use reqwest::{self, header::{USER_AGENT}};

pub mod models;

const SPOOFED_USER_AGENT_HEADER: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36";

pub struct LobstersClient;


impl LobstersClient {
    pub async fn fetch_tags_async(&self) -> Vec<Tag> {
        let client = reqwest::Client::new();
        let endpoint = String::from("https://lobste.rs/tags.json");
        let raw_response = client.get(endpoint)
            .header(USER_AGENT, SPOOFED_USER_AGENT_HEADER)
            .send()
            .await
            .unwrap();
        
        raw_response.json::<Vec<Tag>>().await.unwrap()
    }

    pub async fn fetch_recent_submissions(&self) -> Vec<UserSubmission> {
        let client = reqwest::Client::new();
        let endpoint = String::from("https://lobste.rs/active.json");
        let raw_response = client.get(endpoint)
            .header(USER_AGENT, SPOOFED_USER_AGENT_HEADER)
            .send()
            .await
            .unwrap();
        
        raw_response.json::<Vec<UserSubmission>>().await.unwrap()
    }

    pub async fn fetch_submissions_on_page(&self, page: i32) -> Vec<UserSubmission> {
        let client = reqwest::Client::new();
        let endpoint = format!("https://lobste.rs/page/{}.json", page);
        let raw_response = client.get(&endpoint)
            .header(USER_AGENT, SPOOFED_USER_AGENT_HEADER)
            .send()
            .await
            .unwrap();
        println!("Page {} @ {}", page, endpoint);
        raw_response.json::<Vec<UserSubmission>>().await.unwrap()
    }

    pub async fn fetch_lobster(&self, username: &str) -> Result<Lobster, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let endpoint = format!("https://lobste.rs/u/{}.json", username);
        let raw_response = client.get(endpoint)
            .header(USER_AGENT, SPOOFED_USER_AGENT_HEADER)
            .send()
            .await?;
        
        Ok(raw_response.json::<Lobster>().await?)
    }

    pub async fn build_user_tree(&self) -> HashMap<String, Vec<String>> {
        let mut user_tree_map: HashMap<String, Vec<String>>= HashMap::new();

        // Fetch top 4 pages of submissions on lobste.rs
        let recent_submissions: Vec<UserSubmission> = self.fetch_submissions_on_page(0).await;

        let mut users: Vec<Lobster> = Vec::new();
        for submission in recent_submissions {
            users.push(submission.submitter_user.clone());
        }
                                                                   
        for user in users {
            let mut next_lobster = user;

            loop {
                let (key, invited_by) = (&next_lobster.username, &next_lobster.invited_by_user);
                if user_tree_map.contains_key(invited_by) {
                    let children = user_tree_map.get_mut(invited_by).unwrap();
                    if !children.contains(key) {
                        user_tree_map.get_mut(invited_by)
                                        .unwrap()
                                        .push(key.to_string());
                    }
                } else {
                    user_tree_map.insert(invited_by.to_string(), vec![key.to_string()]);
                }

                println!("{}->{}", key, invited_by);
                if invited_by == "jcs" { break; }
                next_lobster = self.fetch_lobster(invited_by).await.unwrap();
                thread::sleep(time::Duration::from_millis(1500));
            }
            println!("----");
        }

        user_tree_map
    }
}