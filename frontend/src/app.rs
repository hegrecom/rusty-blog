use crate::{
    error_template::{AppError, ErrorTemplate},
    home_page::home_page::HomePage,
};
use leptonic::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    console_error_panic_hook::set_once();

    view! {
        <Stylesheet id="leptos" href="/pkg/rusty-blog-frontend.css"/>

        // sets the document title
        <Title text="Teekey.dev Blog"/>

        // content for this welcome page
        <Root default_theme=LeptonicTheme::Light>
            <Router fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! { <ErrorTemplate outside_errors/> }.into_view()
            }>
                <main>
                    <Routes>
                        <Route path="" view=HomePage/>
                    </Routes>
                </main>
            </Router>
        </Root>
    }
}
