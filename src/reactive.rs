use topcoat::{
    Result,
    context::{Cx, app_context},
    router::page,
    runtime::{Event, procedure, shard},
    view::view,
};

use crate::{Todo, Todos};

// ---------- procedures: async server functions callable from the browser ----------

#[procedure]
async fn double(value: f64) -> Result<f64> {
    Ok(value * 2.0)
}

#[procedure]
async fn add_todo(cx: &Cx, title: String) -> Result<f64> {
    let title = title.trim().to_owned();
    let mut list = app_context::<Todos>(cx).0.lock().unwrap();
    if !title.is_empty() {
        let id = list.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        list.push(Todo { id, title, done: false });
    }
    Ok(list.len() as f64)
}

// ---------- shards: server-rendered components that refresh when their args change ----------

const CRATES: [&str; 8] = ["axum", "serde", "tokio", "topcoat", "tower", "tracing", "hyper", "reqwest"];

#[shard]
async fn search_results(query: String) -> Result {
    let q = query.to_lowercase();
    let hits: Vec<_> = CRATES.iter().filter(|c| c.contains(&q)).collect();
    view! {
        <p class="muted">(hits.len()) " match(es) for “" (query.as_str()) "”"</p>
        <ul class="plain">
            for name in hits {
                <li><code>(*name)</code></li>
            }
        </ul>
    }
}

#[shard]
async fn todo_list(cx: &Cx, version: f64) -> Result {
    let rows: Vec<(u32, String, bool)> = app_context::<Todos>(cx)
        .0
        .lock()
        .unwrap()
        .iter()
        .map(|t| (t.id, t.title.clone(), t.done))
        .collect();
    view! {
        <ul class="plain" data-version=(version)>
            for (id, title, done) in rows {
                <li>
                    // A plain HTML form posting to a #[route]; see api.rs.
                    <form method="post" action=(format!("/todos/{id}/toggle"))>
                        <button>(if done { "undo" } else { "done" })</button>
                    </form>
                    <span class=(done.then_some("done"))>(title)</span>
                </li>
            }
        </ul>
    }
}

// ---------- pages ----------

#[page("/reactive")]
async fn reactive() -> Result {
    view! {
        <h1>"Reactivity"</h1>
        <p class="muted">"Signals and "<code>"$(...)"</code>" expressions run in the browser: no wasm, no round trip."</p>

        <section class="card">
            <h2>"signal + @click"</h2>
            signal count = 0.0;
            <div class="row">
                <button @click=$(|_e| count.decrement())>"−"</button>
                <strong>$(count.get())</strong>
                <button @click=$(|_e| count.increment())>"+"</button>
                <button @click=$(|_e| count.set(0.0))>"reset"</button>
            </div>
            <p>"Positive? " $(if count.get() > 0.0 { "yes" } else { "no" })</p>
        </section>

        <section class="card">
            <h2>":bind + @input (two-way binding)"</h2>
            signal name = String::new();
            <input :value=$(name.get()) @input=$(|e: Event| name.set(e.target.value)) placeholder="your name">
            <button @click=$(|_e| name.push_str("!"))>"add !"</button>
            <p :hidden=$(name.get().is_empty())>"Hello, " $(name.get()) "! (" $(name.get().len()) " bytes)"</p>
        </section>

        <section class="card">
            <h2>"toggle + :hidden + :disabled"</h2>
            signal open = false;
            <button @click=$(|_e| open.toggle())>$(if open.get() { "hide" } else { "show" })</button>
            <button :disabled=$(!open.get())>"only when shown"</button>
            <p :hidden=$(!open.get())>"Peekaboo."</p>
        </section>

        <section class="card">
            <h2>"raw! JavaScript escape hatch"</h2>
            signal word = String::new();
            <input :value=$(word.get()) @input=$(|e: Event| word.set(e.target.value)) placeholder="type a word">
            <p>"Upper-cased: " $({
                let w = word.get();
                raw!("${w}.toUpperCase()", w.to_uppercase())
            })</p>
        </section>

        <section class="card">
            <h2>"#[procedure]: call the server from an event handler"</h2>
            signal n = 1.0;
            <div class="row">
                <strong>$(n.get())</strong>
                <button @click=$(async |_e| {
                    let d = double(n.get()).await;
                    n.set(d);
                })>"double it (server)"</button>
            </div>
        </section>
    }
}

#[page("/search")]
async fn search() -> Result {
    view! {
        <h1>"Shards"</h1>
        <p class="muted">"The list below is rendered by the server each time "<code>"query"</code>" changes."</p>
        signal query = String::new();
        <input :value=$(query.get()) @input=$(|e: Event| query.set(e.target.value)) placeholder="search crates…">
        search_results(query: $(query.get()))
    }
}

#[page("/todos")]
async fn todos() -> Result {
    view! {
        <h1>"Todos"</h1>
        <p class="muted">
            "App context holds the list; a "<code>"#[procedure]"</code>" adds items and bumps "
            <code>"version"</code>", which re-renders the "<code>"#[shard]"</code>"."
        </p>
        signal title = String::new();
        signal version = 0.0;
        <form class="row" @submit=$(|e: Event| e.prevent_default())>
            <input :value=$(title.get()) @input=$(|e: Event| title.set(e.target.value)) placeholder="new todo">
            <button :disabled=$(title.get().trim().is_empty()) @click=$(async |_e| {
                add_todo(title.get()).await;
                title.set("".to_owned());
                version.increment();
            })>"Add"</button>
        </form>
        todo_list(version: $(version.get()))
    }
}
