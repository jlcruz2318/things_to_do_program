// use chrono::DateTime  For Datetime reference later

use std::collections::HashMap;
use std::error::Error;
use std::io::{stdin,stdout,Write};

// use termion::raw::IntoRawMode;
// use tui::Terminal;
// use tui::backend::TermionBackend;
// use tui::widgets::{Widget, Block, Borders};
// use tui::layout::{Layout, Constraint, Direction};

// Come back and add the way that will be used to navigate around the terminal
enum HotkeyOptions {

}

#[derive(Debug)]
enum TaskStatus {
    Open,
    Deleted,
    Closed
}

#[derive(Debug)]
struct ThingToDO {
    description: String,
    status: TaskStatus,
    // started_date: String, page 88 rust book - come back and make a function that gets the time open
    // updated_date: DateTime, Update this later for postgres - above is a place holder - change that to datetime
}

impl ThingToDO {

    fn testinggg(&self) -> String {
        self.description.to_string()
    }

    fn is_done(&self) -> bool {
        match self.status {
            TaskStatus::Open => false,
            TaskStatus::Deleted => false,
            TaskStatus::Closed => true,
        }
    }
}


fn add_thing_to_do(task_list: &mut Vec<ThingToDO>) {
    println!("You have chosen to add a new task.");

    // Get Task Info
    println!("Please provide a description of the task - ");
    let mut description1 = String::new();
    std::io::stdin().read_line(&mut description1).
        expect("Failed to read line");
    description1.pop();

    let status1 = TaskStatus::Open;

    // Push to list
    task_list.push(ThingToDO {description:description1, status:status1});
}

fn main() {
   
    let mut all_things_list :Vec<ThingToDO> = Vec::new();

    let thing = ThingToDO { description: "test".to_string(), status: TaskStatus::Closed };
    println!("<------------<<<<>>>>----------<<<<>>>------------------------<<<<>>>>----------<<<<>>>---------->>");
    println!("The new thing to do is : {:?}", thing);
    println!("The status is {} - F means it is not 'done' - if the status above is 'Closed' then this should be 'true'", thing.is_done());
    println!("The desc is {}", thing.testinggg());

    println!("Adding new item to master list");
    all_things_list.push(thing);
    println!("<------------<<<<>>>>----------<<<<>>>------------------------<<<<>>>>----------<<<<>>>---------->>");
    println!("The complete list is below");
    println!("{:?}", all_things_list);
    println!("<------------<<<<>>>>----------<<<<>>>------------------------<<<<>>>>----------<<<<>>>---------->>");
    // Add Task
    add_thing_to_do(&mut all_things_list);
    println!("<------------<<<<>>>>----------<<<<>>>------------------------<<<<>>>>----------<<<<>>>---------->>");

    println!("The new list is below");

    for (number, thing_to_do) in all_things_list.iter().enumerate() {
        println!{"{} {}, {:?}", number, thing_to_do.description, thing_to_do.status};
    }
    println!("{:?}", all_things_list);

}











// fn main() -> Result<(), io::Error> {
//     let stdout = io::stdout().into_raw_mode()?;
//     let backend = TermionBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;
//     terminal.draw(|f| {
//         let size = f.size();
//         let block = Block::default()
//             .title("Block")
//             .borders(Borders::ALL);
//         f.render_widget(block, size);
//     })
// }