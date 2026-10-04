use topcoat::{
    Result,
    context::Cx,
    router::{
        error::RouterErrorExt,
        page, path_param, query_params,
    },
    view::{View, attributes, class, component, view},
};

// ---------- components: props, #[default], #[into], child content ----------

#[component]
async fn badge(#[into] label: String, #[default("info")] tone: &str) -> Result {
    view! { <span class=(class!("badge", format!("badge-{tone}")))>(label)</span> }
}

#[component]
async fn card(title: &str, child: View) -> Result {
    view! {
        <section class="card">
            <h2>(title)</h2>
            (child)
        </section>
    }
}

enum Status {
    Draft,
    Published { title: &'static str },
    Archived,
}

/// `/` : the `view!` macro, control flow, `class!`, `attributes!`.
#[page("/")]
async fn home() -> Result {
    let fruits = ["apple", "banana", "cherry"];
    let statuses = [Status::Draft, Status::Published { title: "Hello" }, Status::Archived];
    let busy = true;
    let note: Option<&str> = None;
    let tag = "aside";
    let extra = [("data-a", "1"), ("data-b", "2")];
    // `attributes!` builds a reusable attribute fragment outside any element.
    let attrs = attributes! {
        class="btn"
        type="button"
        aria-label="Spread attributes"
        for (name, value) in extra {
            (name)=(value)
        }
    };

    view! {
        <h1>"Views"</h1>
        <p class="muted">"Everything on this page is rendered on the server by "<code>"view!"</code>"."</p>

        card(title: "Components with props and children",
            badge(label: "default tone")
            " "
            badge(label: "ok", tone: "ok")
            " "
            badge(label: "warn", tone: "warn")
        )

        card(title: "if / else, for, match, let",
            if busy {
                <p>"if: busy is true"</p>
            } else {
                <p>"else branch"</p>
            }
            <ul>
                for (i, fruit) in fruits.iter().enumerate() {
                    <li>(i + 1) ". " (*fruit)</li>
                }
            </ul>
            for status in statuses {
                match status {
                    Status::Draft => <p>"match: draft"</p>,
                    Status::Published { title } => <p>"match: published " (title)</p>,
                    Status::Archived if busy => <p>"match: archived (guard)"</p>,
                    _ => "",
                }
            }
            let shout = fruits[0].to_uppercase();
            <p>"let: " (shout)</p>
        )

        card(title: "Attributes",
            <p>
                <button disabled=(busy)>"disabled (bool attr)"</button>
                <button title=(note)>"title omitted (None)"</button>
                <button
                    class=(class!("btn", "primary" if busy else "secondary"))
                    aria-expanded=(if busy { "true" } else { "false" })
                >"class! + enumerated attr"</button>
                <button (attrs)>"attributes! spread"</button>
            </p>
            <(tag) data-dynamic-name="yes">"dynamic element name: <" (tag) ">"</(tag)>
        )
    }
}

// ---------- routing: path params, query params, errors ----------

const POSTS: [(u32, &str, &str); 3] = [
    (1, "Hello Topcoat", "Pages, layouts and components."),
    (2, "Signals", "State that lives in the browser."),
    (3, "Shards", "Server re-renders on demand."),
];

// Declares `PostId`; a non-numeric segment becomes a 404.
path_param!(post_id: u32, error = not_found);

#[query_params(error = bad_request)]
struct PostsQuery {
    q: Option<String>,
}

#[page("/posts")]
async fn posts(cx: &Cx) -> Result {
    let q = query_params::<PostsQuery>(cx)?.q.clone().unwrap_or_default();
    let shown: Vec<_> = POSTS
        .iter()
        .filter(|(_, title, _)| title.to_lowercase().contains(&q.to_lowercase()))
        .collect();
    view! {
        <h1>"Routing"</h1>
        <p class="muted">"Query params via "<code>"#[query_params]"</code>", path params via "<code>"path_param!"</code>"."</p>
        <form method="get" action="/posts">
            <input name="q" value=(q.as_str()) placeholder="filter posts">
            " "<button>"Filter"</button>
        </form>
        <ul>
            for (id, title, _) in shown {
                <li><a href=(format!("/posts/{id}"))>(*title)</a></li>
            }
        </ul>
        <p><a href="/posts/999">"A missing post (404 caught by the layout)"</a></p>
    }
}

#[page("/posts/{post_id}")]
async fn post(cx: &Cx) -> Result {
    let id = *path_param::<PostId>(cx)?;
    let (_, title, body) = POSTS.iter().find(|p| p.0 == id).ok_or_not_found()?;
    view! {
        <h1>(*title)</h1>
        <p>(*body)</p>
        <p><a href="/posts">"All posts"</a></p>
    }
}
