// use chrono::DateTime  For Datetime reference later


use std::convert::TryInto;
use std::error::Error;
use std::io;
use std::io::{stdin,stdout,Write};

use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, BorderType};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style, Color};

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
    println!("You have chosen to add a new task - ");

    // Get task info
    println!("Please provide a description of the task - ");
    let mut description1 = String::new();
    std::io::stdin().read_line(&mut description1)
        .expect("Failed to read line");
    description1.pop();

    let status1 = TaskStatus::Open;

    // Push to list
    task_list.push(ThingToDO {description:description1, status:status1});
}

fn delete_thing_to_do(task_list: &mut Vec<ThingToDO>) {
    let task_numbers = 0..task_list.len();
    println!("You have chosen to delete a task -");

    // Get task to delete info
    println!("What number is the task in the list?");
    let mut task_number = String::new();
    std::io::stdin().read_line(&mut task_number)
        .expect("Failed to read line");

    // get info from UI list
    let task_number : usize = task_number.trim().parse()
        .expect("Failed to read line");

    //make sure user has provided a valid task number
    if !task_numbers.contains(&task_number) {
        println!("No such task exists - [aborting]");
        return
    }

    //confirm deletion
    println!("Are you sure you want to delete ---- {:?} ---- [Type N to cancel or ANY key to Continue]", task_list[task_number].description);
    let mut confirm_delete = String::new();
    std::io::stdin().read_line(&mut confirm_delete)
        .expect("Failed to read line");    

    if confirm_delete.to_string().trim() == "N".to_string() {
        println!("Not deleting - [aborting]");
        return
    }

    // finally - delete
    task_list.remove(task_number.try_into().unwrap()); //Put un checks before deleting
}

// fn finish_someting(task_list: &mut Vec<ThingToDO>) {
    // finish this later - make this log a completion date - i had questions about editing a Vec<ThingToDo>


fn main() -> Result<(), io::Error>{

    let stdout = std::io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(70),
                    Constraint::Percentage(20),
                ].as_ref()
            )
            .split(f.size());
        let block = Block::default()
             .title("Block")
             .borders(Borders::ALL)
             .border_style(Style::default().fg(Color::White))
             .border_type(BorderType::Rounded);
        f.render_widget(block, chunks[0]);
        let block = Block::default()
             .title("Block 2")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    })?;
    Ok(())
        
    // let mut all_things_list :Vec<ThingToDO> = Vec::new();

    // let thing = ThingToDO { description: "test".to_string(), status: TaskStatus::Closed };
    // println!("---------------------------------------------------------------");
    // println!("The new thing to do is : {:?}", thing);
    // println!("The status is {} - F means it is not 'done' - if the status above is 'Closed' then this should be 'true'", thing.is_done());
    // println!("The desc is {}", thing.testinggg());
    // println!("---------------------------------------------------------------");
    // println!("Adding new item to master list\n\n");
    // all_things_list.push(thing);
    // println!("---------------------------------------------------------------");
    // println!("The complete list is below");
    // println!("{:?}", all_things_list);
    // println!("---------------------------------------------------------------");
    // println!("The new list is below");

    // for (number, thing_to_do) in all_things_list.iter().enumerate() {
    //     println!{"{} {}, {:?}", number, thing_to_do.description, thing_to_do.status};
    // }
    // println!("---------------------------------------------------------------");
    // println!("{:?}", all_things_list);
    // println!("---------------------------------------------------------------");

    // Ok(());

}

