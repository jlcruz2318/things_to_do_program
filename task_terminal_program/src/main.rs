use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView,
                     LinearLayout, SelectView};
use cursive::traits::*;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

fn main() {
    let mut siv = cursive::default();

    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("select")
        .fixed_size((70, 30));
    let buttons = LinearLayout::vertical()
        .child(Button::new("Update", get_things))
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(Dialog::around(LinearLayout::vertical()
            .child(select)
            .child(DummyView)
            .child(buttons))
        .title("Things to do"));
    
    siv.run();
}

fn add_name(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(name)
        });
        s.pop_layer();
    }

    s.add_layer(Dialog::around(EditView::new()
            .on_submit(ok)
            .with_name("name")
            .fixed_width(35))
        .title("Enter a new name")
        .button("Ok", |s| {
            let name =
                s.call_on_name("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));
}

fn delete_name(s: &mut Cursive) {
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}

fn on_submit(s: &mut Cursive, name: &str) {
    s.add_layer(Dialog::text(format!("Name: {}\nAwesome: yes", name))
        .title(format!("{}'s info", name))
        .button("Quit", |s| { s.pop_layer();}));
}

fn get_things(s: &mut Cursive) {
    let file_path = "/home/check0ut/Projects/things_to_do_program/task_terminal_program/src/tasks";
    let contents = std::fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    s.call_on_name("select", |view: &mut SelectView<String>| {
        view.add_item_str(contents)
    });
}


// fn main() {
//     get_things();
// }