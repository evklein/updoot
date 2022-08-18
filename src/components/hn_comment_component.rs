
use updoot::models::hn_request_models::Comment;
use updoot::{HackerNewsClient};
use yew::{html, Component, Context, Html, Properties};
use yew::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use std::sync::{Arc, Mutex};
use yew_hooks::prelude::*;
use yew::html::Scope;

// region

pub struct HNCommentComponent {
    item: i64,
}

#[derive(PartialEq, Properties)]
pub struct CommentComponentProps {
    pub latest_item: i64,
}

pub enum CommentComponentMessage {
    NumberChanged(i64),
}

impl HNCommentComponent {

}

impl Component for HNCommentComponent {
    type Message = CommentComponentMessage;
    type Properties = CommentComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            item: ctx.props().latest_item,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CommentComponentMessage::NumberChanged(value) => {
                self.item = value;
                true
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_future(fetch_number_of_items());
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div class="columns">
                <div class="column">
                    <p>{ self.item }</p>
                </div>
            </div>
            </>
        }
    }
}

async fn fetch_number_of_items() -> CommentComponentMessage {
    let client = HackerNewsClient {};
    let response = client.get_latest_item_id().await;
    CommentComponentMessage::NumberChanged(response.unwrap())
}