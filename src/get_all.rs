use colored::Colorize;
use std::fs;

use crate::todo::TodoItem;

pub fn get_all(args: &[String]) {
    let data = fs::read_to_string("./data.json").unwrap_or("[]".to_string());
    let todos: Vec<TodoItem> = serde_json::from_str(&data).expect("data.json格式不正确");

    for i in 0..todos.len() {
        if todos[i].done
            && (args.contains(&String::from("-u")) || args.contains(&String::from("-unfinished")))
        {
            continue;
        }
        if !todos[i].done
            && (args.contains(&String::from("-f")) || args.contains(&String::from("-finished")))
        {
            continue;
        }

        let checkbox: &str = match todos[i].done {
            true => "✅",
            false => "❎",
        };

        print!(
            "{} {}{}{} {} {} {}   ",
            checkbox,
            todos[i].name.bold(),
            " ".repeat(25 - todos[i].name.chars().count()),
            "ID".bold(),
            i.to_string(),
            "创建时间".bold(),
            todos[i].time.bold()
        );
        for j in 0..todos[i].tags.len() {
            match todos[i].tags[j].as_str() {
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
        println!();
    }
}
