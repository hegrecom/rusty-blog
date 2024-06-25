use leptonic::prelude::*;
use leptos::*;

use crate::{
    backend_client::client::BackendClient,
    post::post_card::PostCard,
};

#[component]
pub fn HomePage() -> impl IntoView {
    let posts_resource = create_resource(|| (), |_| async move {
        BackendClient::client().fetch_posts(1, 10).await
    });

    view! {
        <Grid spacing=Size::Px(0)>
            <Suspense fallback=move || {
                view! { <div>Loading...</div> }
            }>
                {move || {
                    posts_resource
                        .get()
                        .map(|posts_response| {
                            match posts_response {
                                Ok(posts) => {
                                    posts
                                        .data()
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
                                }
                                Err(e) => view! { <div>{e.to_string()}</div> }.into_view(),
                            }
                        })
                }}

            </Suspense>
        </Grid>
    }
}
