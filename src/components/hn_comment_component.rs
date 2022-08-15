use updoot::models::hn_request_models::Comment;
use yew::{Component, Context, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
    comment: Comment,
}

pub struct HNCommentComponent;

impl Component for HNCommentComponent {
    type Message = ();
    type Properties = Props;
    
    fn create(ctx: &Context<Self>) -> Self {
        HNCommentComponent
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="columns">
                    <div class="column">
                        {ctx.props().comment.by }{ ":" }{ctx.props().comment.text}
                    </div>
                </div>
            </>
        }
    }
}