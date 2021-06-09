use std::convert::TryInto;
use std::error::Error;
use std::io;
use std::io::{stdin,stdout,Write};

use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders, List, ListItem, ListState};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Color, Modifier, Style};

// Come back and add the way that will be used to navigate around the terminal
enum HotkeyOptions {
}


fn add_thing_to_do(task_list: &mut Vec<ThingToDO>) {
    println!("You have chosen to add a new task - ");

    // Get task info
    println!("Please provide a description of the task - ");
    let mut description1 = String::new();
    std::io::stdin().read_line(&mut description1)
        .expect("Failed to read line");
    description1.pop();

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




// TUI - LIST APP STRUCT //
/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.

fn main() -> Result<(), std::io::Error>{

    let stdout = std::io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut state = ListState::default();
    loop {
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

            let mut all_things_list :Vec<ThingToDO> = Vec::new();
            let thing = ThingToDO { description: "test".to_string(), status: TaskStatus::Closed };
            all_things_list.push(thing);

            // Create a List from all list items and highlight the currently selected one
            let items = vec![
                ListItem::new("Task1"),
                ListItem::new("Task2"),];

            
            let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("List"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

            // We can now render the item list
            f.render_stateful_widget(list, chunks[0], &mut state);



            // let block = Block::default()
            //     .title("Block")
            //     .borders(Borders::ALL)
            //     .border_style(Style::default().fg(Color::White))
            //     .border_type(BorderType::Rounded);
            // f.render_widget(block, chunks[0]);
        })?;
    }
}




    // for (number, thing_to_do) in all_things_list.iter().enumerate() {
    //     println!{"{} {}, {:?}", number, thing_to_do.description, thing_to_do.status};
    // }
    // println!("---------------------------------------------------------------");
    // println!("{:?}", all_things_list);
    // println!("---------------------------------------------------------------");

    // Ok(());


