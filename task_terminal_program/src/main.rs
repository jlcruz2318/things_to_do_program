use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView,
                     LinearLayout, SelectView};
use cursive::traits::*;
use cursive::theme::{Color, PaletteColor};

use std::cell::{RefCell, RefMut};
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::rc::Rc;

fn main() {
    let mut siv = cursive::default();
    // Theme
    let mut theme = siv.current_theme().clone();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    siv.set_theme(theme);

    // UI
    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("select")
        .fixed_size((70, 30));
    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add_todo))
        .child(Button::new("Delete", delete_todo))
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(Dialog::around(LinearLayout::vertical()
            .child(select)
            .child(DummyView)
            .child(buttons))
        .title("Things to do"));
    
    // Load existing todo list - this will eventually tie to a server
    let todo_items: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let mut todo_items = load_existing_todos(&mut siv, todo_items);

    siv.run();
    println!("{:?}", todo_items)
}

fn load_existing_todos(s: &mut Cursive, todo_list: Rc<RefCell<Vec<String>>>) {
    // Create vec for the to do items
    let mut todo_list: RefMut<_> = todo_list.borrow_mut();
    // For now this will be a static path - later need to update to a server request
    let file_path = "/tasks";
    let contents = read_lines(file_path)
        .expect("Something went wrong reading the file");
    s.call_on_name("select", |view: &mut SelectView<String>| {
        for line in contents {
            if let Ok(task) = line {
                view.add_item_str(&task);
                todo_list.push(task);
            }
        }
    
    });
}

// TODO once this updates - it has to push new tasks to the database
// TODO Track the order of the way things are updated with timestamps
// TODO Store the items locally - and then push to redis - delete clean up after - run a test often to reconcile
fn add_todo(s: &mut Cursive) {
    
    fn ok(s: &mut Cursive, name: &str)  {
        s.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(name);

            let new_list = view.iter();
            let file_path = "/tasks";
            let mut file = File::create(file_path).unwrap();

            for task in new_list {
                writeln!(&mut file, "{}", task.1).unwrap()
            }
            
        });
        s.pop_layer();
    }



    s.add_layer(Dialog::around(EditView::new()
            .on_submit(ok)
            .with_name("new_todo")
            .fixed_width(35))
        .title("Enter a new to do task")
        .button("Ok", |s| {
            let name =
                s.call_on_name("new_todo", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            // run fn ok
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));
}

fn delete_todo(s: &mut Cursive) {
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No task to remove")),
        Some(focus) => {
            select.remove_item(focus);
            let new_list = select.iter();
            let file_path = "/tasks";
            let mut file = File::create(file_path).unwrap();
            for task in new_list {
                writeln!(&mut file, "{}", task.1).unwrap();
                }
            ;
            
        }
    }
}

fn on_submit(s: &mut Cursive, name: &str) {
    s.add_layer(Dialog::text(format!("Name: {}", name))
        .title(format!("{}'s info", name))
        .button("Quit", |s| { s.pop_layer();}));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}