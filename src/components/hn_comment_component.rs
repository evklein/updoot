use updoot::models::hn_request_models::Comment;
use yew::prelude::*;
use yew::{html, Context, Html, Properties};

pub struct HNCommentComponent {
    comment: Comment,
}

#[derive(PartialEq, Properties)]
pub struct CommentComponentProps {
    pub comment: Comment,
}

pub enum CommentComponentMessage {
    SetComment(Comment),
}

impl Component for HNCommentComponent {
    type Message = CommentComponentMessage;
    type Properties = CommentComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            comment: ctx.props().comment.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CommentComponentMessage::SetComment(new_comment) => {
                if self.comment.id == new_comment.id {
                    return false;
                }

                self.comment = new_comment;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div class="column">
                <div class="box">
                    <div class="comment-text title is-6">
                        { &self.comment.text }
                    </div>
                    <div class="comment-bottom">
                        <p><i class="fas fa-user"></i>{ " " }{ &self.comment.by }</p>
                        <p><i class="fas fa-clock"></i>{ " " }{ self.comment.time }</p>
                    </div>
                </div>
            </div>
            </>
        }
    }
}

// async fn fetch_number_of_items() -> CommentComponentMessage {
//     let client = HackerNewsClient {};
//     let response = client.get_latest_item_id().await;
//     CommentComponentMessage::NumberChanged(response.unwrap())
// }
