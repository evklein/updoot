use updoot::models::hn_request_models::Comment;
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub comment: Comment,
}

pub struct HNCommentComponent;

impl Component for HNCommentComponent {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        HNCommentComponent
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="columns">
                    <div class="column">
                        {&ctx.props().comment.by }{ ": " }{&ctx.props().comment.text}
                    </div>
                </div>
            </>
        }
    }
}
