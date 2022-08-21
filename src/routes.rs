use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/play")]
    Game,
    #[at("/tree")]
    Tree,
    #[not_found]
    #[at("/404")]
    NotFound,
}