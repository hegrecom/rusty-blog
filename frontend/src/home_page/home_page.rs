use leptonic::prelude::*;
use leptos::*;

use crate::{
    backend_client::client::BackendClient, post::{
        post::Post,
        post_card::PostCard,
    }
};

#[component]
pub fn HomePage() -> impl IntoView {
    let (page, set_page) = create_signal(1);
    let (posts, set_posts): (ReadSignal<Vec<Post>>, WriteSignal<Vec<Post>>) = create_signal(vec![]);
    let posts_resource = create_resource(
        move || page.get(),
        |page| async move {
            let per = 10;
            logging::log!("fetching posts for page: {}", page);
            match BackendClient::default().fetch_posts(page, per).await {
                Ok(post_response) => {
                    post_response.data()
                }
                Err(_e) => vec![],
            }
        },
    );

    window_event_listener(ev::scrollend, move |_| {
        logging::log!("scrollend");
        set_page.update(|page| *page += 1);
    });

    view! {
        <Grid spacing=Size::Px(0)>
            <Transition fallback=move || {}>
                {move || {
                    posts_resource
                        .get()
                        .map(|fetched_posts| {
                            set_posts.update(|posts| posts.extend(fetched_posts));
                            posts
                                .get()
                                .into_iter()
                                .map(|post| {
                                    view! {
                                        <Row>
                                            <Col md=2 sm=2 xs=0>
                                                <div></div>
                                            </Col>
                                            <Col md=8 sm=8 xs=12 style="justify-content: center;">
                                                <PostCard post=post/>
                                            </Col>
                                            <Col md=2 sm=2 xs=0>
                                                <div></div>
                                            </Col>
                                        </Row>
                                    }
                                })
                                .collect_view()
                                .into_view()
                        })
                }}

            </Transition>
            {move || { if posts_resource.loading().get() { "loading..." } else { "" } }}

        </Grid>
    }
}
