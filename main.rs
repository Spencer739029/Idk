use axum::{
    routing::{get, post},
    extract::Form,
    Router,
    response::{Html, Redirect, IntoResponse},
};
use tokio::fs;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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
        .route("/hello", post(hello))
        .route("/after", post(after))
        .nest_service("/static", ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind failed");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

async fn hello(Form(_form): Form<HashMap<String, String>>) -> impl IntoResponse {
    Html(format!(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Document</title>
        </head>
        <body>
            <h1>That's cool!</h1>
            <form action="/after" method="post">
                <label for="gender">Gender:</label>
                <select name="gender" id="gender">
                    <option value="male">Male</option>
                    <option value="female">Female</option>
                    <option value="prefer">Prefer not to say</option>
                    <option value="other">Other</option>
                </select>
                <label for="mood">How are you feeling?</label>
                <select name="mood" id="mood">
                    <option value="happy">Happy</option>
                    <option value="sad">Sad</option>
                    <option value="angry">Angry</option>
                    <option value="jealous">Jealous</option>
                    <option value="excited">Excited</option>
                    <option value="motivated">Motivated</option>
                    <option value="confused">Confused</option>
                    <option value="tired">Tired</option>
                </select>

                <button type="submit">Submit</button>
            </form>
        </body>
        </html>
    "#))
}

async fn name_age(Form(form): Form<HashMap<String, String>>) -> impl IntoResponse {
    if let (Some(name), Some(surname), Some(id)) = (form.get("name"), form.get("surname"), form.get("id")) {
        let path = "submissions.json";

        let data = fs::read_to_string(path)
            .await
            .unwrap_or_else(|_| "[]".to_string());

        let mut submissions: Vec<Submission> = serde_json::from_str(&data)
            .unwrap_or_default();

        let id_value = id.parse::<i128>().unwrap_or(0);

        let entry = Submission {
            name: name.to_string(),
            surname: surname.to_string(),
            id: id_value,
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
        let html = format!(r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Document</title>
            </head>
            <body>
                <h1>Hello {}!</h1>
                <form action="/hello" method="post">
                    <label for="age">Age:</label>
                    <input type="number" name="age" id="age">

                    <button type="submit">Submit</button>
                </form>
            </body>
            </html>
            "#,
        name);

        Html(html)
    } else {
        let html = format!(r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Document</title>
            </head>
            <body>
                <h1>No name provided</h1>
                <a href="/">Go back</a>
            </body>
            </html>
            "#);
        Html(html)
    }
}

async fn after() {
    
}