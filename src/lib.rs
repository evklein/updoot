use std::{collections::HashMap, hash};

use std::{thread, time};
use models::lobsters_request_models::{UserSubmission, Tag, Lobster};
use reqwest::{self, header::{USER_AGENT}};

pub mod models;

pub struct LobstersClient;

impl LobstersClient {
    pub async fn fetch_tags_async(&self) -> Vec<Tag> {
        let client = reqwest::Client::new();
        let endpoint = String::from("https://lobste.rs/tags.json");
        let raw_response = client.get(endpoint)
            //.header(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
            //.header(ACCEPT_LANGUAGE, "en-US,en;q=0.9")
            //.header(COOKIE, "lobster_trap=SUs5ZOOv0Q%2Bs7OQ%2BKN5u5Oekp3EIRLnGSnLtduYy2QFKoFVfX8Mlegd2kk26%2FHk2%2Bri7wfWQ7hv7W830tbE7lfJmNw0epckP7xAwOKPAwz3eWEs6a9Aw4eTnBoNZ2W1JXiND2T3RIkOGMmBMXOf6bMoCYXrNNh5yG76gDXfTSnaSH0%2FEKKNEQEPjRXjlBPj2nnL5erAbgzQR%2FnJ%2FWWxRE1WZrkHxnPZelftC0AMqSgMpJJbYPO%2BNWS4ouAlCrN4E7Qvo2DPgQtEaZm%2FJDLsbIZQqsRLjiY%2BK5AcrPzk5TRif36AXX9XkjFue--jym6yiUvhcRqN7%2BK--Wty%2F2CchHEV729Sgrpr5Vg%3D%3D")
            //.header(ACCEPT_ENCODING, "gzip, deflate, br")
            //.header(CACHE_CONTROL, "no-cache")
            //.header(PRAGMA, "no-cache")
            //.header(UPGRADE_INSECURE_REQUESTS, "1")
            .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36")
            .send()
            .await
            .unwrap();
        
        raw_response.json::<Vec<Tag>>().await.unwrap()
    }

    pub async fn fetch_recent_submissions(&self) -> Vec<UserSubmission> {
        let client = reqwest::Client::new();
        let endpoint = String::from("https://lobste.rs/active.json");
        let raw_response = client.get(endpoint)
            .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36")
            .send()
            .await
            .unwrap();
        
        raw_response.json::<Vec<UserSubmission>>().await.unwrap()
    }

    pub async fn fetch_recent_submissions_for_page(&self, client: &reqwest::Client, page: i32) -> Vec<UserSubmission> {
        let endpoint = format!("https://lobste.rs/page/{}.json", page);
        let raw_response = client.get(endpoint)
            .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36")
            .send()
            .await
            .unwrap();
        println!("Page{}", page);
        raw_response.json::<Vec<UserSubmission>>().await.unwrap()
    }

    pub async fn fetch_recent_submissions_range(&self, number_of_pages: i32) -> Vec<UserSubmission> {
        let client = reqwest::Client::new();
        let mut submissions: Vec<UserSubmission> = Vec::new();

        for page_index in 1..number_of_pages {
            if page_index == 7 { continue; }
            let mut next_submission_set = self.fetch_recent_submissions_for_page(&client, page_index).await;
            submissions.append(next_submission_set.as_mut());
        }

        submissions
    }

    pub async fn fetch_lobster(&self, username: &str) -> Result<Lobster, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let endpoint = format!("https://lobste.rs/u/{}.json", username);
        let raw_response = client.get(endpoint)
            .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36")
            .send()
            .await?;
        
        Ok(raw_response.json::<Lobster>().await?)
    }

    pub async fn build_user_tree(&self) -> HashMap<String, Vec<String>> {
        let mut userTreeMap: HashMap<String, Vec<String>>= HashMap::new();
        let client = reqwest::Client::new();

        // Fetch top 4 pages of submissions on lobste.rs
        let recent_submissions: Vec<UserSubmission> = self.fetch_recent_submissions_range(4).await;

        // For each User submission, get the User info
        for submission in recent_submissions.iter() {
            println!("what {}", &submission.submitter_user.username);
            let next_lobster = self.fetch_lobster(&submission.submitter_user.username).await.unwrap();
            println!("{}", next_lobster.username);
            thread::sleep(time::Duration::from_millis(500));
        }

        userTreeMap
    }
}