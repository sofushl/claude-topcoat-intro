use topcoat::{
    Result,
    context::{Cx, memoize},
    cookie::{Cookies, cookies},
    router::{
        Slot, StatusCode,
        error::NotFoundError,
        layout,
        request::uri,
    },
    view::{View, class, component, error_boundary, view},
};

/// `#[memoize]`: computed once per request, however many components ask.
#[memoize]
pub async fn theme(cx: &Cx) -> String {
    match cookies(cx).get("theme") {
        Some(c) if c.value() == "dark" => "dark".to_owned(),
        _ => "light".to_owned(),
    }
}

/// Wraps every page. `slot` is the page (or nested layout) being rendered.
#[layout("/")]
async fn root_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html data-theme=(theme(cx).await)>
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"Topcoat tour"</title>
                <link rel="stylesheet" href="/style.css">
                topcoat::runtime::script()
                topcoat::dev::script()
            </head>
            <body>
                <nav>
                    nav_link(href: "/", label: "Views")
                    nav_link(href: "/reactive", label: "Reactivity")
                    nav_link(href: "/search", label: "Shards")
                    nav_link(href: "/todos", label: "Todos")
                    nav_link(href: "/posts", label: "Routing")
                    nav_link(href: "/request", label: "Request & cookies")
                </nav>
                <main>
                    // A layout can catch a page's error and replace it with a branded page.
                    error_boundary(
                        fallback: |error| {
                            if error.downcast_ref::<NotFoundError>().is_none() {
                                return Err(error);
                            }
                            Ok(view! {
                                (StatusCode::NOT_FOUND)
                                <h1>"Page not found"</h1>
                                <p><a href="/">"Back home"</a></p>
                            })
                        },
                        (slot)
                    )
                </main>
            </body>
        </html>
    })
}

#[component]
async fn nav_link(cx: &Cx, href: &str, label: &str) -> Result<impl View> {
    let path = uri(cx).path();
    let current = if href == "/" { path == "/" } else { path.starts_with(href) };
    Ok(view! {
        <a href=(href) class=(class!("active" if current)) aria-current=(current.then_some("page"))>
            (label)
        </a>
    })
}
