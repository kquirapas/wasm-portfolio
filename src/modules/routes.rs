use yew_router::prelude::*;

// Routing
#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    // add new routes like this
    // #[at("/blog")]
    // Blog,
    #[not_found]
    #[at("/404")]
    NotFound,
}
