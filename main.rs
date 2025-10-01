use axum::{
    routing::{get, post},
    extract::Form,
    Router,
    response::{Html, Redirect},
};
use tokio::fs;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Submission {
    name: String,
    surname: String,
    id: i128,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(index))
    .route("/NameAge", post(name_age))
    .route("/submissions", get(show_submissions))
    .nest_service("/static", ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind failed");

    axum::serve(listener, app.into_make_service())
        .await
        .expect("server failed");
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

// Changed return type to explicitly be Redirect
async fn name_age(Form(form): Form<Submission>) -> Redirect {
    let path = "submissions.json";

    let data = fs::read_to_string(path)
        .await
        .unwrap_or_else(|_| "[]".to_string());

    let mut submissions: Vec<Submission> = serde_json::from_str(&data)
        .unwrap_or_default();

    let mut rng = rand::rng();

    let entry = Submission {
        name: form.name,
        surname: form.surname,
        id: rng.random_range(1..=9999),
    };
    
    submissions.push(entry);

    let json = serde_json::to_string_pretty(&submissions)
        .unwrap_or_else(|err| {
            eprintln!("Failed to serialize submissions: {}", err);
            "[]".to_string()
        });

    if let Err(err) = tokio::fs::write(path, json).await {
        eprintln!("Failed to write submissions: {}", err);
    }

    Redirect::to("/submissions")
}

async fn show_submissions() -> Html<String> {
    let path = "submissions.json";
    let data = fs::read_to_string(path).await.unwrap_or_else(|_| "[]".to_string());
    let submissions: Vec<Submission> = serde_json::from_str(&data).unwrap_or_default();

    let mut html = String::from("<h1>Submissions</h1><ul>");
    for s in submissions {
        html.push_str(&format!("<li>{} {} (ID: {})</li>", s.name, s.surname, s.id));
    }
    html.push_str("</ul><a href=\"/\">Go back</a>");

    Html(html)
}