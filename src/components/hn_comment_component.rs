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
            comment: ctx.props().comment.to_owned(),
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
            <div class="comment-text title is-6">
                { &self.comment.text }
            </div>
            <div class="comment-bottom">
                <p><i class="fas fa-user"></i>{ " " }{ &self.comment.by }</p>
                <p><i class="fas fa-clock"></i>{ " " }{ self.comment.time }</p>
                <p><i class="fas fa-hashtag"></i>{ " "}{self.comment.id }</p>
            </div>
            </>
        }
    }
}
