use std::any;

use anyhow::Result;
use leptos::*;
use libsql_client::Row;

#[derive(Debug, Clone)]
pub struct TodoItem {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

fn pluck_string(row: &Row, pos: usize) -> Result<String> {
    use libsql_client::Value::*;

    return match &row.values[pos] {
        Text { value } => Ok(value.clone()),
        _ => Err(anyhow::anyhow!("yo dawg, this is no string")),
    };
}

fn pluck_me_daddy_int(row: &Row, pos: usize) -> Result<usize> {
    use libsql_client::Value::*;
    return match row.values[pos] {
        Integer { value } => Ok(value as usize),
        _ => Err(anyhow::anyhow!("yo dawg, this is no integer")),
    };
}

impl TryFrom<Row> for TodoItem {
    type Error = anyhow::Error;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        let id = pluck_me_daddy_int(&row, 0)?;
        let title = pluck_string(&row, 1)?;
        let completed = pluck_me_daddy_int(&row, 2)?;

        return Ok(TodoItem {
            id,
            title,
            completed: completed == 1,
        });
    }
}

#[component]
pub fn TodoForm(cx: Scope, todos: Vec<TodoItem>, route: &'static str) -> impl IntoView {
    // create user interfaces with the declarative `view!` macro
    return view! { cx,
        <form hx-post="{route}"
            hx-target="#todos"
            hx-swap="afterbegin"
            hx-trigger="submit">
            <input type="text" placeholder="More like Todone, amirite?" />
            <button type="submit">"add me daddy"</button>
            <Todos todos=todos />
        </form>
    };
}

#[component]
pub fn Todos(cx: Scope, todos: Vec<TodoItem>) -> impl IntoView {
    let (todos, _) = create_signal::<Vec<TodoItem>>(cx, todos);

    // create user interfaces with the declarative `view!` macro
    return view! { cx,
        <ul id="todos">
            <For

                // a function that returns the items we're iterating over; a signal is fine
                each=move || todos.get()

                // a unique key for each item

                key=|todo| todo.id

                // renders each item to a view
                view=move |cx, todo: TodoItem| {
                    view! {
                        cx,
                        <Todo todo=todo />
                    }
                }
            />
        </ul>
    };
}

#[component]
pub fn Todo(cx: Scope, todo: TodoItem) -> impl IntoView {
    return view! {cx,
        <div>
            <div>id: {todo.id}</div>
            <div>title: {todo.title}</div>
            <div>completed: {todo.completed}</div>
        </div>
    };
}
