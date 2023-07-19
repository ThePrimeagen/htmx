mod todo;

use anyhow::Result;
use libsql_client::{Client, Config, Value};
use std::sync::Arc;
use todo::TodoForm;
use todo::{Todo, TodoItem};

use actix_web::{get, middleware, post, web, App, HttpRequest, HttpResponse, HttpServer};
use leptos::*;

#[derive(Clone)]
struct AppState {
    client: Arc<Client>,
}

async fn get_count(client: Arc<Client>) -> Result<usize> {
    let count = client.execute("SELECT COUNT(*) FROM todos").await?;
    let count = count
        .rows
        .first()
        .map(|row| &row.values[0])
        .unwrap_or(&Value::Integer { value: 0 });
    let count = match count {
        Value::Integer { value: i } => *i,
        _ => 0,
    };

    return Ok(count as usize);
}

#[get("/")]
async fn index(_req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let todos = data
        .client
        .execute("SELECT * FROM todos").await.unwrap()
        .rows
        .iter()
        .filter_map(|x| TodoItem::try_from(x.clone()).ok())
        .collect::<Vec<_>>();

    println!("todos: {:?}", todos);

    let html = leptos::ssr::render_to_string(move |cx| {
        view! { cx,
            <head>
                <script src="https://unpkg.com/htmx.org@1.9.2" integrity="sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h" crossorigin="anonymous"></script>
            </head>
            <body>
                <TodoForm
                    route="/todo"
                    todos=vec![
                        TodoItem { id: 0, title: "hello".to_string(), completed: false },
                        TodoItem { id: 1, title: "world".to_string(), completed: true },
                    ]
                />
            </body>
        }
    });

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
}

#[post("/todo")]
async fn create_todo(_req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    // me daddy
    let what_comes_back = data
        .client
        .execute("INSERT INTO todos (title, completed) VALUES ('me daddy', 1) RETURNING id")
        .await
        .unwrap();
    println!(" TODO FIX ME PRIME DADDY {:?}", what_comes_back);

    let html = leptos::ssr::render_to_string(move |cx| {
        view! { cx,
            <Todo
                todo=TodoItem { id: 69, title: "world".to_string(), completed: true }
            />
        }
    });

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
}

fn get_url() -> String {
    let file = std::env::var("DB").unwrap_or("/tmp/example.db".to_string());

    if file.starts_with("file://") {
        return file;
    }

    return format!("file://{}", file);
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::new(get_url().as_str())?;
    let client = Arc::new(libsql_client::Client::from_config(config).await.unwrap());

    client.execute("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, completed BOOLEAN)").await.unwrap();

    println!("count {}", get_count(client.clone()).await?);

    let app_state = web::Data::new(AppState { client });

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .app_data(app_state.clone())
            .service(index)
            .service(create_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    return Ok(());
}
