use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path;
use std::process;

#[derive(Serialize, Deserialize)]
struct TodoItem {
    name: String,
    completed: char,
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        TodoItem {
            name,
            completed: ' ',
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { list: Vec::new() }
    }

    fn init() -> TodoList {
        if path::Path::new(".todos").exists() {
            let saved_todos = fs::read_to_string(".todos").expect("Unable to read file");
            let todo_list: TodoList = serde_json::from_str(&saved_todos).unwrap();
            return todo_list;
        } else {
            TodoList::new()
        }
    }

    fn write(&self) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        fs::write(".todos", json).expect("Unable to write file");
    }

    fn add_to_list(&mut self, name: String) {
        let todo_item = TodoItem::new(name);
        self.list.push(todo_item);
    }

    fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!("{}: [{}] - {}", index, item.completed, item.name);
        }
    }

    fn toggle_done(&mut self, index: usize) {
        if self.list[index].completed == ' ' {
            self.list[index].completed = 'X';
        } else {
            self.list[index].completed = ' ';
        }
    }

    fn remove_task(&mut self, index: usize) {
        self.list.remove(index);
    }

    fn clear_list(&mut self) {
        self.list.clear();
    }

    fn clean_completed(&mut self) {
        self.list.retain(|item| item.completed == ' ');
    }
}

enum Command {
    Get,
    Clear,
    Clean,
    Help,
    Add(String),
    Done(usize),
    Remove(usize),
}

fn help() {
    println!("usage: todo <command> [<args>]
    get             returns all todo items
    clear           clear all todo items
    clean           remove all completed todo items
    add <task>      create a new todo item
    done <index>    mark todo item as done
    remove <index>  remove todo item");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut todo_list = TodoList::init();

    if args.len() == 1 {
        help();
        process::exit(0);
    }

    let command = match args[1].as_str() {
        "get" => Command::Get,
        "clear" => Command::Clear,
        "clean" => Command::Clean,
        "add" => Command::Add(args[2..].join(" ")),
        "done" => Command::Done(args[2].parse().expect("Error converting to int")),
        "remove" => Command::Remove(args[2].parse().expect("Error converting to int")),
        _ => Command::Help,
    };

    match command {
        Command::Get => todo_list.print(),
        Command::Clear => {
            todo_list.clear_list();
            todo_list.write();
        }
        Command::Clean => {
            todo_list.clean_completed();
            todo_list.write();
            todo_list.print();
        }
        Command::Help => {
            help();
        }
        Command::Add(task) => {
            todo_list.add_to_list(task);
            todo_list.write();
            todo_list.print();
        }
        Command::Done(i) => {
            todo_list.toggle_done(i);
            todo_list.write();
            todo_list.print();
        }
        Command::Remove(i) => {
            todo_list.remove_task(i);
            todo_list.write();
            todo_list.print();
        }
    }
}
