use axum::{
    routing::{get, post},
    extract::Form,
    Router,
    response::{Html, Redirect},
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
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/NameAge", post(name_age))
        .nest_service("/static", ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server failed");
}


async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

async fn name_age(Form(form): Form<HashMap<String, String>>) -> impl IntoResponse {
    if let Some(name) = form.get("name") {
        if let Some(surname) = form.get("surname") {
            let path = "submissions.json";

            let data = fs::read_to_string(path)
                .await
                .unwrap_or_else(|_| "[]".to_string());

            let mut submissions: Vec<Submission> = serde_json::from_str(&data)
                .unwrap_or_default();

            if let (Some(name), Some(surname)) = (form.get("name"), form.get("surname")) {
                let entry = Submission {
                    name: name.to_string(),
                    surname: surname.to_string(),
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
            }


            Redirect::to("/submissions")

        } else {
            Html(r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <title>No Name</title>
                </head>
                <body>
                    <h1>No surname entered</h1>
                    <a href="/">Go back to start</a>
                </body>
                </html>
            "#.to_string())
        }
    } else {
        Html(r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <title>No Name</title>
            </head>
            <body>
                <h1>No name entered</h1>
                <a href="/">Go back to start</a>
            </body>
            </html>
        "#.to_string())
        }
    }