use yew::{Html, function_component, html};

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
        <>
        <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
        <div class="navbar-brand p-4">
            <div class="has-text-white">
                <i class="fa fa-arrow-up title is-2 has-text-white"></i>
                <span class="title is-2 has-text-white" style="font-family: 'Gotham Rounded'">{ " Updoot " }</span>
                <p class="subtitle is-6 has-text-white ">{ "A Rust demonstration" }</p>
            </div>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
        </div>

        <div id="navbarBasicExample" class="navbar-menu">
            <div class="navbar-start">
            <a href="/" class="navbar-item">
                { "Home" }
            </a>

            <a href="/play" class="navbar-item">
                { "Play" }
            </a>

            <a href="/tree" class="navbar-item">
                { "Tree" }
            </a>
            </div>
            <div class="navbar-end">
            <a href="https://github.com/evklein/updoot" class="navbar-item title is-5">
                <i class="fa-brands fa-github pr-2" />
                { "Repository" }
            </a>
            </div>
        </div>
        </nav>          
        </>
    }
}
