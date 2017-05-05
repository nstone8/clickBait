extern crate gtk;
use gtk::prelude::*;

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
    let my_box=Paned::new(Orientation::Vertical);
    let play_box=Box::new(Orientation::Horizontal,10);
    let scroll=ScrolledWindow::new(None,None);
    play_box.add(&rew_button);
    play_box.add(&play_button);
    play_box.add(&ff_button);
    scroll.add(&image);
    my_box.add(&scroll);
    my_box.add(&play_box);
    window.add(&my_box);
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
struct Control {}

impl Control {
    fn play(&self){
        println!("Play")
    }

    fn rew(&self){
        println!("Rewind")
    }

    fn ff(&self){
        println!("Fast Forward")
    }
}
impl Clone for Control{
    fn clone(&self)->Self{
        return Control{}
    }
}
impl Copy for Control {}
