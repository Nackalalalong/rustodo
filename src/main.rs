use std::fs::read_to_string;
use std::fs::File;
use std::io;
use std::path::Path;

enum MainPageOption {
    ViewDoneTasks,
    ViewTodoTasks,
    AddTask,
    Exit
}

enum Page {
    Main,
    DoneTasks,
    TodoTasks,
    AddTask
}

fn create_file_if_not_exist(path: &str) {
    if !Path::new(path).exists() {
        match File::create(path) {
            Err(why) => panic!("couldn't create {}: {}", path, why),
            Ok(file) => file,
        };
    }
}

fn read_data(path: &str) -> Vec<String> {
    create_file_if_not_exist(path);

    let content = read_to_string(path).expect("could not read file");

    return content.split('\n').map(str::to_string).filter(|e| e.len() > 0).collect::<Vec<String>>();
}

fn clear_screen(){
    print!("\x1B[2J\x1B[1;1H");
}

fn save_tasks(todo_tasks: &Vec<String>, done_tasks: &Vec<String>){

}

fn show_main_page() -> MainPageOption {
    clear_screen();
    print!("Welcome to RUSTODO!
    
(1) view todo tasks
(2) view done tasks
(3) add task

(0) exit
");

    let mut input_text = String::new();

    let mut num: i32 = -1;

    while num < 0 {
        io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    
        let trimmed = input_text.trim();

        num = match trimmed.parse::<i32>() {
            Ok(i) => i,
            Err(..) => -1,
        };

        if num > 3 {
            num = -1;
        }

        input_text.clear();
    }

    if num == 1 {
        return MainPageOption::ViewTodoTasks;
    }
    else if num == 2 {
        return MainPageOption::ViewDoneTasks;
    }
    else if num == 3 {
        return MainPageOption::AddTask;
    }

    return MainPageOption::Exit;
}

fn show_todo_tasks_page(tasks: &Vec<String>) -> usize {
    clear_screen();

    for (i,task) in tasks.iter().enumerate() {
        println!("({}) {}", i + 1, task);
    }
    println!();
    println!("enter number corresponding the a task to mark as complete, otherwise go to main page");

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    
        let trimmed = input_text.trim();

    let num = match trimmed.parse::<usize>() {
        Ok(i) => i,
        Err(..) => 0,
    };

    return num;
}

fn show_done_tasks_page(tasks: &Vec<String>){
    clear_screen();

    for (i,task) in tasks.iter().enumerate() {
        println!("({}) {}", i + 1, task);
    }
    println!();
    println!("enter any keys to go to main page");

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
}

fn main() {

    let mut page = Page::Main;
    let mut running = true;
    let mut input_text = String::new();

    let done_path = "done.txt";
    let todo_path = "todo.txt";

    let mut done_tasks = read_data(done_path);
    let mut todo_tasks = read_data(todo_path);

    while running {
        match page {
            Page::Main => {
                match show_main_page() {
                    MainPageOption::ViewDoneTasks => {
                        page = Page::DoneTasks;
                    },
                    MainPageOption::ViewTodoTasks => {
                        page = Page::TodoTasks;         
                    },
                    MainPageOption::AddTask => {
                        page = Page::AddTask;
                    }
                    MainPageOption::Exit => {
                        running = false;
                    }
                };
            },
            Page::DoneTasks => {
                show_done_tasks_page(&done_tasks);
                page = Page::Main;
            },
            Page::TodoTasks => {
                let num = show_todo_tasks_page(&todo_tasks);
                if num == 0 {
                    page = Page::Main;
                }
                else {
                    let task = todo_tasks.remove(num - 1);
                    done_tasks.push(task);
                }
            },
            Page::AddTask => {
                clear_screen();
                input_text.clear();
                println!("enter description:");
                io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
                todo_tasks.push(input_text.trim().to_string());

                page = Page::TodoTasks;
            }
        }
    }

    save_tasks(&todo_tasks, &done_tasks);

}   
