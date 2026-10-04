use topcoat::{
    Result,
    context::{Cx, memoize},
    cookie::{Cookies, cookies},
    router::{
        StatusCode,
        error::NotFoundError,
        layout,
        request::uri,
    },
    view::{class, component, view},
};

/// `#[memoize]`: computed once per request, however many components ask.
#[memoize]
pub async fn theme(cx: &Cx) -> String {
    match cookies(cx).get("theme") {
        Some(c) if c.value() == "dark" => "dark".to_owned(),
        _ => "light".to_owned(),
    }
}

/// Wraps every page. `slot` is the page (or nested layout) already rendered.
#[layout("/")]
async fn root_layout(cx: &Cx, slot: Result) -> Result {
    // A layout can catch a page's error and replace it with a branded page.
    let content = match slot {
        Err(e) if e.downcast_ref::<NotFoundError>().is_some() => view! {
            (StatusCode::NOT_FOUND)
            <h1>"Page not found"</h1>
            <p><a href="/">"Back home"</a></p>
        },
        other => other,
    }?;

    view! {
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
                <main>(content)</main>
            </body>
        </html>
    }
}

#[component]
async fn nav_link(cx: &Cx, href: &str, label: &str) -> Result {
    let path = uri(cx).path();
    let current = if href == "/" { path == "/" } else { path.starts_with(href) };
    view! {
        <a href=(href) class=(class!("active" if current)) aria-current=(current.then_some("page"))>
            (label)
        </a>
    }
}
