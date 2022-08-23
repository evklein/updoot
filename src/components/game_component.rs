use updoot::{models::hn_request_models::Comment, HackerNewsClient};
use web_sys::HtmlInputElement;
use yew::{Component, Context, Html, Properties};
use yew::html;
use rand::Rng;

use crate::components::hn_comment_component::HNCommentComponent;


#[derive(PartialEq, Properties)]
pub struct GameComponentProps {}

pub enum GameComponentMessage {
    IncrementUserScore,
    IncrementComputerScore,
    RefreshEligibleParents(Vec<Comment>),
    RefreshChildComments,
    CheckWinner,
}

pub struct GameComponent {
    loading_hn_data: bool,
    user_score: usize,
    computer_score: usize,
    eligible_parent_comments: Vec<Comment>,
    current_parent_comment: Comment,
    //current_child_comments: (Comment, Comment),
}

impl Component for GameComponent {
    type Message = GameComponentMessage;
    type Properties = GameComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        GameComponent {
            loading_hn_data: true,
            user_score: 0,
            computer_score: 0,
            eligible_parent_comments: Vec::new(),
            current_parent_comment: Comment::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameComponentMessage::IncrementUserScore => {
                self.user_score += 1;
                true
            }
            GameComponentMessage::IncrementComputerScore => {
                self.computer_score += 1;
                true
            }
            GameComponentMessage::RefreshEligibleParents(parent_comments) => {
                self.eligible_parent_comments = parent_comments;

                let mut rng = rand::thread_rng();
                let index_of_parent: usize = rng.gen_range(0..self.eligible_parent_comments.len());
                if let Some(comment) = self.eligible_parent_comments.get(index_of_parent) {
                    self.current_parent_comment = comment.to_owned();
                } else {
                    self.current_parent_comment = Comment { by: "woah!!".to_owned(), id: 0, kids: Vec::new(), parent: 0, text: String::new(), time: 0, item_type: String::new() }
                }

                self.loading_hn_data = false;
                true
            },
            GameComponentMessage::RefreshChildComments => true,
            GameComponentMessage::CheckWinner => true,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.loading_hn_data {
            html! { 
                <p>
                    <i class="fas fa-spinner fa-pulse mr-2"></i>
                    { "Loading game data..." }
                </p>
            }
        } else {
            html! {
                <>
                    <p>{ "Number of items: " }{ self.eligible_parent_comments.len() }</p>
                    {
                        if self.current_parent_comment.time != 0 {
                            html! {
                                <HNCommentComponent comment={self.current_parent_comment.to_owned()} />
    
                            }
                        } else { html! {}}
                    }
                </>
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_future(fetch_initial_items());
        }
    }
}

async fn fetch_initial_items() -> GameComponentMessage {
    let client = HackerNewsClient {};
    let response = client.get_latest_items(150).await;

    let comments = match response {
        Ok(res) => res.comments,
        Err(_) => Vec::new(),
    };

    let eligible_comments: Vec<Comment> = comments
        .into_iter()
        .filter(|comment| comment.kids.len() >= 1)
        .collect();

    GameComponentMessage::RefreshEligibleParents(eligible_comments)
}
