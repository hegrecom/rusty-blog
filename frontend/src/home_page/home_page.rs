use leptonic::prelude::*;
use leptos::*;

use crate::post::{post::Post, post_card::PostCard};

#[component]
pub fn HomePage() -> impl IntoView {
    let post = Post {
        title: "Hello, World!".to_string(),
        content: r#"
        There are many stories to tell, but this is the first one.
        Write Some stories here.
        Once upon a time, there was a developer who wanted to write a blog.
        He started writing a blog and he was happy.
        And then he wrote another blog post.
        And then he wrote another blog post.
        And then he wrote another blog post.
        But he was not happy.
        He wanted to write more blog posts.
        Why?
        Because he wanted to write more blog posts.
        And then he wrote another blog post.
        And then he wrote another blog post.
        And then he wrote another blog post.
        And then he wrote another blog post.
        It was totally worth it.
        He messed up the blog post.
        And then he wrote another blog post.
        "#
        .to_string(),
        created_at: chrono::Local::now(),
        updated_at: chrono::Local::now(),
    };

    view! {
        <Grid spacing=Size::Px(
            0,
        )>

            {(0..10)
                .map(|_| {
                    let cloned_post = post.clone();
                    view! {
                        <Row>
                            <Col md=3 sm=2 xs=0>
                                <div></div>
                            </Col>
                            <Col md=6 sm=8 xs=12>
                                <PostCard post=cloned_post/>
                            </Col>
                            <Col md=3 sm=2 xs=0>
                                <div></div>
                            </Col>
                        </Row>
                    }
                })
                .collect_view()}
        </Grid>
    }
}
