use std::env;

struct TodoItem {
    name: String,
    completed: char
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem{
            name: name,
            completed: ' '
        };
    }
}


struct TodoList {
    list: Vec<TodoItem>
}

impl TodoList {
    fn new() -> TodoList {
        return TodoList{
            list: Vec::new()
        };
    }

    fn add_to_list(&mut self, name: String) {
        self.list.push(TodoItem::new(name));
    }

    fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!("{} [{}] - {}", index, item.completed, item.name);
        }
    }

    fn mark_done(&mut self, index: usize) {
        if self.list[index].completed == ' ' {
            self.list[index].completed = 'x';
        } else {
            self.list[index].completed = ' ';
        }
    }

    fn remove_task(&mut self, index: usize) {
        self.list.remove(index);
    }
}

enum Command {
    Get,
    Add(String),
    Done(usize),
    Remove(usize)

}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut todo_list = TodoList::new();

    let command = match args[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(args[2].clone()),
        "done" => Command::Done(args[2].parse().expect("Error converting to integer.")),
        "remove" => Command::Remove(args[2].parse().expect("Error converting to integer.")),
        _ => panic!("You must provide an accepted commands.")
    };

    todo_list.add_to_list("Say Good Morning to you.".to_string());
    todo_list.add_to_list("Get the Tom's MacBook Air at the shop.".to_string());
    todo_list.mark_done(1);

    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add_to_list(task);
            todo_list.print();
        },
        Command::Done(index) => {
            todo_list.mark_done(index);
            todo_list.print();
        },
        Command::Remove(index) => {
            todo_list.remove_task(index);
            todo_list.print();

        }
    }
}