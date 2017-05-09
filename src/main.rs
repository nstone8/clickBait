extern crate gtk;
use gtk::prelude::*;
mod click_object;
use click_object::{Control,LabelFrame,Attachable};

use gtk::{Button, Window, WindowType, Image,Orientation,Box,ScrolledWindow,Paned};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    
    let window = Window::new(WindowType::Toplevel);
    window.set_title("clickbait");
    window.set_default_size(350, 70);
    let play_button = Button::new_with_label("Play/Pause");
    let rew_button = Button::new_with_label("Rewind");
    let ff_button = Button::new_with_label("Fast Forward");
    
    let image=Image::new_from_file("/home/nstone/Desktop/Advisor email.png");
    let top_box=Paned::new(Orientation::Horizontal);
    let my_box=Paned::new(Orientation::Vertical);
    let play_box=Box::new(Orientation::Horizontal,10);
    let image_scroll=ScrolledWindow::new(None,None);
    
    play_box.add(&rew_button);
    play_box.add(&play_button);
    play_box.add(&ff_button);
    image_scroll.add(&image);
    my_box.add(&image_scroll);
    my_box.add(&play_box);
    top_box.add(&my_box);
    let labels=LabelFrame::new();
    labels.attach(&top_box);
    window.add(&top_box);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    static C: Control = Control{};
    play_button.connect_clicked(|_| {C.play()});
    rew_button.connect_clicked(|_| {C.rew()});
    ff_button.connect_clicked(|_| {C.ff()});

    gtk::main();
}
