use crate::todo::TodoItem;
use chrono::Local;
use colored::Colorize;
use std::fs;

pub fn new_todo(name: String) {
    let data = fs::read_to_string("./data.json").unwrap_or("[]".to_string());
    let mut todos: Vec<TodoItem> =
        serde_json::from_str(&data).expect("data.json does not have correct format");
    let now = Local::now();
    let now_format = now.format("%Y-%m-%d %H:%M:%S").to_string();

    todos.push(TodoItem {
        done: false,
        name: name,
        tags: vec![],
        time: now_format,
    });

    let seroalized = serde_json::to_string_pretty(&todos);
    fs::write("./data.json", seroalized.unwrap()).expect("写入失败");

    println!(
        "{}{}{} {}",
        todos[todos.len() - 1].name.bold(),
        " ".repeat(25 - todos[todos.len() - 1].name.chars().count()),
        "ID:".bold(),
        (todos.len() - 1).to_string()
    );
}
