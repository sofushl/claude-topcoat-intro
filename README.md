# Claude Topcoat Intro

A tour of [Topcoat](https://github.com/tokio-rs/topcoat) (0.10): one small working example per major feature.

## Run

```sh
cargo install topcoat-cli --version 0.10.0 --locked  # needs rustc 1.98
topcoat dev                                          # build, bundle assets, serve, live-reload
# or, without live reload:
topcoat asset bundle && cargo run
```

Open <http://127.0.0.1:3000> (override with `HOST` / `PORT`). The asset bundle is required because
Topcoat's browser runtime script is served as a bundled asset.

## What's where

| Page / route | Feature | File |
|---|---|---|
| `/` | `view!`, `#[component]` (props, `#[default]`, child content), `if`/`for`/`match`/`let`, conditional attributes, `class!`, `attributes!`, dynamic element names | `src/pages.rs` |
| `/reactive` | `signal`, `$()` expressions, `@event` and `:bind` attributes, `toggle`/`increment`/`push_str`, `raw!`, `#[procedure]` | `src/reactive.rs` |
| `/search` | `#[shard]` re-rendered server-side as a signal changes | `src/reactive.rs` |
| `/todos` | app context, procedure + shard, form `POST` route + `see_other` | `src/reactive.rs`, `src/api.rs` |
| `/posts`, `/posts/{id}` | `path_param!`, `#[query_params]`, `ok_or_not_found`, 404 caught in a layout | `src/pages.rs` |
| `/request` | request helpers, cookie jar, `Form` | `src/api.rs` |
| `/api/health`, `POST /api/echo` | `#[route]`, `Json`, `#[layer]` | `src/api.rs` |
| everywhere | `#[layout]`, `#[memoize]`, status codes, `Css` route, router setup | `src/layout.rs`, `src/main.rs` |

Not covered: sessions, assets/fonts/icons, mail, Tailwind, htmx/Datastar (all are opt-in features or need extra setup).
