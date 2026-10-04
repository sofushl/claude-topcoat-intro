use serde::{Deserialize, Serialize};
use topcoat::{
    Result,
    context::{Cx, app_context},
    cookie::{Cookie, Cookies, cookies},
    router::{
        Body, Next, HeaderValue,
        content::{Form, Json},
        error::{RouterErrorExt, SeeOther, see_other},
        layer, page, path_param,
        request::{headers, method, uri},
        response::Response,
        route,
    },
    view::view,
};

use crate::Todos;

// ---------- a layer: wraps every request under /api ----------

#[layer("/api")]
async fn timing(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let start = std::time::Instant::now();
    let mut response = next.run(cx, body).await?;
    let micros = start.elapsed().as_micros().to_string();
    response.headers_mut().insert("x-elapsed-us", HeaderValue::from_str(&micros).unwrap());
    Ok(response)
}

// ---------- JSON routes ----------

#[route(GET "/api/health")]
async fn health() -> Result<&'static str> {
    Ok("ok")
}

#[derive(Serialize, Deserialize)]
struct Echo {
    message: String,
}

/// `curl -X POST localhost:3000/api/echo -H 'content-type: application/json' -d '{"message":"hi"}'`
#[route(POST "/api/echo")]
async fn echo(Json(input): Json<Echo>) -> Result<Json<Echo>> {
    Ok(Json(Echo { message: input.message.to_uppercase() }))
}

// ---------- forms + redirects ----------

path_param!(todo_id: u32, error = not_found);

#[route(POST "/todos/{todo_id}/toggle")]
async fn toggle_todo(cx: &Cx) -> Result<SeeOther> {
    let id = *path_param::<TodoId>(cx)?;
    let mut todos = app_context::<Todos>(cx).0.lock().unwrap();
    let todo = todos.iter_mut().find(|t| t.id == id).ok_or_not_found()?;
    todo.done = !todo.done;
    Ok(see_other("/todos"))
}

#[derive(Deserialize)]
struct ThemeForm {
    theme: String,
}

#[route(POST "/theme")]
async fn set_theme(cx: &Cx, Form(form): Form<ThemeForm>) -> Result<SeeOther> {
    let value = if form.theme == "dark" { "dark" } else { "light" };
    cookies(cx).add(Cookie::build(("theme", value)).path("/").build());
    Ok(see_other("/request"))
}

// ---------- request helpers + cookies on a page ----------

#[page("/request")]
async fn request(cx: &Cx) -> Result {
    let theme = crate::layout::theme(cx).await;
    let user_agent = headers(cx)
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    let next = if theme == "dark" { "light" } else { "dark" };
    view! {
        <h1>"Request & cookies"</h1>
        <section class="card">
            <h2>"Request helpers"</h2>
            <pre>
                (method(cx).to_string()) " " (uri(cx).to_string()) "\n"
                "user-agent: " (user_agent)
            </pre>
        </section>
        <section class="card">
            <h2>"Cookie jar"</h2>
            <p>"Current theme cookie: "<code>(theme.as_str())</code></p>
            <form method="post" action="/theme">
                <input type="hidden" name="theme" value=(next)>
                <button>"Switch to " (next)</button>
            </form>
        </section>
        <section class="card">
            <h2>"API routes"</h2>
            <p><a href="/api/health">"GET /api/health"</a>" (see the "<code>"x-elapsed-us"</code>" header added by a layer)"</p>
            <pre>"curl -X POST localhost:3000/api/echo -H 'content-type: application/json' -d '{\"message\":\"hi\"}'"</pre>
        </section>
    }
}
