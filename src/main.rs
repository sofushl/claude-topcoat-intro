//! A tour of Topcoat: every major feature gets a small, working example.
//!
//! | file          | covers                                                                  |
//! |---------------|-------------------------------------------------------------------------|
//! | `main.rs`     | router setup, app context, cookies, asset bundle, `route` + `Css`       |
//! | `layout.rs`   | `#[layout]`, `#[memoize]`, cookies, status codes, request helpers       |
//! | `pages.rs`    | `view!`, `#[component]`, `class!`, `attributes!`, `path_param!`, queries |
//! | `reactive.rs` | signals, `$()`, `@`/`:` attributes, `#[procedure]`, `#[shard]`          |
//! | `api.rs`      | `#[route]`, `#[layer]`, `Json`, `Form`, `see_other`, cookies            |

mod api;
mod layout;
mod pages;
mod reactive;

use std::sync::Mutex;

use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    cookie::RouterBuilderCookieExt,
    router::{Router, RouterBuilderDiscoverExt, content::Css, route},
};

/// Shared across requests via app context (see `Router::app_context` below).
pub struct Todos(pub Mutex<Vec<Todo>>);

pub struct Todo {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

#[tokio::main]
async fn main() {
    let router = Router::builder()
        // Collects every #[page], #[layout], #[layer], #[route], #[procedure], #[shard].
        .discover()
        // `runtime::script()` is served as a bundled asset: run `topcoat dev` or
        // `topcoat asset bundle` first.
        .assets(AssetBundle::load().expect("asset bundle missing: run `topcoat asset bundle`"))
        .cookies()
        .app_context(Todos(Mutex::new(vec![
            Todo { id: 1, title: "Read the Topcoat docs".into(), done: true },
            Todo { id: 2, title: "Build something".into(), done: false },
        ])))
        .build();

    // Binds HOST:PORT, default 127.0.0.1:3000.
    topcoat::start(router).await.unwrap();
}

#[route(GET "/style.css")]
async fn stylesheet() -> Result<Css<&'static str>> {
    Ok(Css(include_str!("style.css")))
}
