use yew_router::prelude::*;
use yew::prelude::*;

// root (global) modules
mod modules;
mod pages;

use modules::routes::Route;

use pages::{
    home::Home,
    not_found::NotFound
};

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {<Home/>},
        Route::NotFound => html! {<NotFound/>}
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    // for console logging
    // remove if no console logging needed
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}
