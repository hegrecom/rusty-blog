use crate::{
    error_template::{AppError, ErrorTemplate},
    post::{post::Post, post_card::PostCard},
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
        <Title text="Welcome to Leptos"/>

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

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);

    let post = Post {
        title: "Hello, World!".to_string(),
        content: "Welcome to Leptos!".to_string(),
        created_at: chrono::Local::now(),
        updated_at: chrono::Local::now(),
    };

    view! {
        <Box style="display: flex; flex-direction: column; align-items: center; padding: 1em; min-height: 100%; min-width: 100%">
            <H2>"Welcome to Leptonic"</H2>

            <span style="margin-top: 3em;">"Count: " {move || count.get()}</span>
            <Button on_click=move |_| set_count.update(|c| *c += 1)>"Increase"</Button>
            <PostCard post=post/>
        </Box>
    }
}
