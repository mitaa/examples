extern crate gtk;

use gtk::{IconSize, Orientation, ReliefStyle};
use gtk::signal::Inhibit;
use gtk::traits::*;

struct NoteBook {
    notebook: gtk::NoteBook,
    tabs: Vec<gtk::Box>
}

impl NoteBook {
    fn new() -> NoteBook {
        NoteBook {
            notebook: gtk::NoteBook::new().unwrap(),
            tabs: Vec::new()
        }
    }

    fn create_tab<'a, Widget: gtk::WidgetTrait>(&mut self, title: &'a str, widget: &Widget) ->
            Option<u32> {
        let close_image = gtk::Image::new_from_icon_name("window-close", IconSize::Button).unwrap();
        let button = gtk::Button::new().unwrap();
        let label = gtk::Label::new(title).unwrap();
        let tab = gtk::Box::new(Orientation::Horizontal, 0).unwrap();

        button.set_relief(ReliefStyle::None);
        button.set_focus_on_click(false);
        button.add(&close_image);

        tab.pack_start(&label, false, false, 0);
        tab.pack_start(&button, false, false, 0);
        tab.show_all();

        let index = match self.notebook.append_page(widget, Some(&tab)) {
            Some(index) => index,
            _ => return None
        };

        let notebook_clone = self.notebook.clone();
        button.connect_clicked(move |_| notebook_clone.remove_page(index as i32));

        self.tabs.push(tab);

        Some(index)
    }
}

fn main() {
    gtk::init();

    let window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();

    window.set_title("Notebook");
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(640, 480);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    let mut notebook = NoteBook::new();

    for i in 1..4 {
        let title = format!("sheet {}", i);
        let label = gtk::Label::new(&title[..]).unwrap();
        notebook.create_tab(&title[..], &label);
    }

    window.add(&notebook.notebook);
    window.show_all();
    gtk::main();
}

