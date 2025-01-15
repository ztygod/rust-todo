use colored::Colorize;
use std::fs;

use crate::todo::TodoItem;

pub fn get_one(index: usize) {
    let data = fs::read_to_string("./data.json").unwrap_or("[]".to_string());
    let todos: Vec<TodoItem> = serde_json::from_str(&data).expect("data.json格式不正确");

    if todos.get(index).is_none() {
        println!("索引为 {} todo 没有找到", index);
        return;
    }

    let checkbox: char = match todos[index].done {
        true => '✅',
        false => '❌',
    };

    println!("{} {}", "序号：".bold(), index);
    println!("{} {}", "todo:".bold(), todos[index].name.as_str().bold());
    println!("{} {}", "是否完成".bold(), checkbox);
    println!(
        "{} {}",
        "建立时间：".bold(),
        todos[index].time.as_str().bold()
    );
    print!("{}", "标签：".bold());

    for i in 0..todos[index].tags.len() {
        match todos[index].tags[i].as_str() {
            "red" => print!("{} ", "⦿".red()),
            "yellow" => print!("{} ", "⦿".yellow()),
            "green" => print!("{} ", "⦿".green()),
            "blue" => print!("{} ", "⦿".blue()),
            "cyan" => print!("{} ", "⦿".cyan()),
            "magenta" => print!("{} ", "⦿".magenta()),
            "purple" => print!("{} ", "⦿".purple()),
            _ => print!("?"),
        }
    }
}
