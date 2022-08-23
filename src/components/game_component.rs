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
    SelectNewParent,
    RefreshChildComments(Comment, Comment),
    CheckWinner,
}

pub struct GameComponent {
    loading_hn_data: bool,
    user_score: usize,
    computer_score: usize,
    eligible_parent_comments: Vec<Comment>,
    current_parent_comment: Comment,
    current_child_comments: (Comment, Comment),
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
            current_child_comments: (Comment::new(), Comment::new()),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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

                let index_of_child: usize = rng.gen_range(0..self.current_parent_comment.kids.len());

                ctx.link().send_future(fetch_child_comments(self.current_parent_comment.kids[index_of_child]));
                true
            },
            GameComponentMessage::SelectNewParent => {
                self.loading_hn_data = true;

                let mut rng = rand::thread_rng();
                let index_of_parent: usize = rng.gen_range(0..self.eligible_parent_comments.len());
                if let Some(comment) = self.eligible_parent_comments.get(index_of_parent) {
                    self.current_parent_comment = comment.to_owned();
                } else {
                    self.current_parent_comment = Comment { by: "woah!!".to_owned(), id: 0, kids: Vec::new(), parent: 0, text: String::new(), time: 0, item_type: String::new() }
                }

                let index_of_child: usize = rng.gen_range(0..self.current_parent_comment.kids.len());
                ctx.link().send_future(fetch_child_comments(self.current_parent_comment.kids[index_of_child]));
                true
            }
            GameComponentMessage::RefreshChildComments(comment_a, comment_b) => {
                self.current_child_comments = (comment_a, comment_b);
                self.loading_hn_data = false;
                true
            },
            GameComponentMessage::CheckWinner => true,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="m-6">
                <p class="title is-4">{ "Choose the fake comment to win!" }</p>
                <hr />
                <div class="tags has-addons">
                    <span class="tag is-primary">{ "You: "}{self.user_score}</span>
                    <span class="tag is-danger">{ "Computer: "}{self.computer_score}</span>
                </div>
                {
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
                            {
                                if self.current_parent_comment.time != 0 {
                                    html! {
                                        <>
                                        <HNCommentComponent comment={self.current_parent_comment.clone()} />
                                        <div class="columns">
                                            <div class="column">
                                                <HNCommentComponent comment={self.current_child_comments.0.clone()} />
                                            </div>
                                            <div class="column">
                                                <HNCommentComponent comment={self.current_child_comments.1.clone()} />
                                            </div>
                                        </div>
                                        </>
                                    }
                                } else { html! {}}
                            }
                        </>
                    }
                }
            }
            </div>
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

async fn fetch_child_comments(comment_id: i64) -> GameComponentMessage {
    // Grab real child comment.
    let client = HackerNewsClient {};
    let response = client.get_comment_by_id(comment_id).await;

    let legit_comment = match response {
        Ok(res) => res,
        Err(_) => Comment::new(),
    };

    // Grab GPT-3 generated comment.

    GameComponentMessage::RefreshChildComments(legit_comment, Comment::new())
}