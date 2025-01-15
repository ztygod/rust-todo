use colored::Colorize;
use std::fs;

use crate::todo::TodoItem;

pub fn remove(index: usize) {
    let data = fs::read_to_string("./data.json").unwrap_or("[]".to_string());
    let mut todos: Vec<TodoItem> = serde_json::from_str(&data).expect("data.json 格式不正确");

    if todos.get(index).is_none() {
        println!("索引为{}不存在", index);
        return;
    }

    let checkbox: char = match todos[index].done {
        true => '✅',
        false => ' ',
    };

    print!(
        "{} {}{}{} {} {} {}",
        checkbox,
        todos[index].name.bold(),
        " ".repeat(25 - todos[index].name.chars().count()),
        "ID:".bold(),
        index.to_string(),
        "创建时间".bold(),
        todos[index].time.bold(),
    );
    for j in 0..todos[index].tags.len() {
        match todos[index].tags[j].as_str() {
            "red" => print!("{} ", "⦿".red()),
            "yellow" => print!("{} ", "⦿".yellow()),
            "green" => print!("{} ", "⦿".green()),
            "blue" => print!("{} ", "⦿".blue()),
            "cyan" => print!("{} ", "⦿".cyan()),
            "magenta" => print!("{} ", "⦿".magenta()),
            "purple" => print!("{} ", "⦿".purple()),
            _ => print!("颜色错误"),
        }
    }
    print!("   {}", "已删除".red());

    todos.remove(index);
    let serialized = serde_json::to_string_pretty(&todos);
    fs::write("data.json", serialized.unwrap()).expect("写入失败");
}
