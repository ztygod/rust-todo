use colored::Colorize;
use std::{env, fs};

pub mod todo;
pub extern crate colored;
pub extern crate serde;
pub extern crate serde_json;

pub mod get_one;
use get_one::get_one;

pub mod new_todo;
use new_todo::new_todo;

pub mod get_all;
use get_all::get_all;

pub mod finish;
use finish::finish;

pub mod remove;
use remove::remove;

pub mod tags;
use tags::tags;
fn main() {
    //println!("{} {} !", "it".green(), "works".blue().bold());
    let args: Vec<String> = env::args().collect();

    match &*args[1] {
        "--help" | "-h" => print_help(),
        "--get" | "-g" => get_one(args[2].parse().unwrap()),
        "--list" | "-l" => get_all(&args[2..args.len()]),
        "--new" | "-n" => new_todo(args[2..args.len()].join(" ").clone()),
        "-done" | "-d" => finish(args[2].parse().unwrap()),
        "--remove" | "-r" => remove(args[2].parse().unwrap()),
        "--clear" | "-c" => {
            fs::write("data.json", "[]").unwrap();
            println!("todo已清空")
        }
        "--color" | "-cl" => tags(
            args[2]
                .parse()
                .expect(&format!("Index '{}' invalid", args[2])),
            args[3].parse().unwrap(),
        ),
        "-version" | "-v" => println!("rust-todo v{}", env!("CARGO_PKG_VERSION")),
        _ => println!("没有相关命令"),
    }
}

fn print_help() {
    println!("一个用rust编写的todo小项目🎊🎊🎊");
    println!();
    println!("命令行输入规范:");
    println!("     rust-todo [OPTIONS]");
    println!("     rust-todo [OPTIONS] [ARGUMENTS]");
    println!();
    println!("命令行选项");
    println!("     --help, -h                      提供所有帮助消息");
    println!("     --get, -g    [index]            通过index索引，找到todo项");
    println!("     --list, -l   [ARGUMENTS]        打印所有todo列表");
    println!("                  --fininshed. -f    打印所有已完成的todo列表");
    println!("                  --unfinished, -u   打印所有未完成的todo列表");
    println!("     --new, -n    [NAME]             输入命名，新建todo项");
    println!("     --clear, -c                     清空所有todo列表");
    println!("     --remove, -r [index]            通过index索引，删除指定todo列表");
    println!("     --color, -cl [index] [COLOR]    通过index索引，个性化todo项颜色");
    println!("     --done, -d   [index]            完成索引处todo列表");
    println!("     -version, -v                    输出版本号");
    println!();
    println!("可选颜色：");
    println!("    {}", "red ⦿".red());
    println!("    {}", "blue ⦿".blue());
    println!("    {}", "yellow ⦿".yellow());
    println!("    {}", "green ⦿".green());
    println!("    {}", "cyan ⦿".cyan());
    println!("    {}", "purple, magenta ⦿".purple());
}
