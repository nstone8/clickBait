extern crate gtk;

use gtk::prelude::*;

mod click_object;
use click_object::{Control,LabelFrame,Attachable};

use std::path::PathBuf;

use gtk::{Button, WindowType, Image,Orientation,Box,ScrolledWindow,Paned,FileChooserDialog,DialogExt,FileChooserAction,ResponseType,Window};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
//Select file to open
	 let chooser_window=Window::new(WindowType::Toplevel);
	 let chooser=FileChooserDialog::new::<Window>(Some("Open Video"),Some(&chooser_window),FileChooserAction::Open);
	 chooser.add_button("Open",ResponseType::Ok.into());
	 chooser.add_button("Cancel",ResponseType::Cancel.into());
	 let filename:PathBuf;
	 let response_code=chooser.run();
		if response_code==ResponseType::Ok.into(){
				//We've got a file, let's open it
				 filename=chooser.get_filename().unwrap();
				 chooser.destroy();
		}else if response_code == ResponseType::Cancel.into(){
		      	 //if they don't want to open a file, what are we doing
				     println!("No file, no clickbait");
				     return;
			} else {
			//eeek
		     println!("Unknown signal from open dialog");
		     return;
		}


    let window = Window::new(WindowType::Toplevel);
    window.set_title("clickbait");
    window.set_default_size(350, 70);
    let play_button = Button::new_with_label("Play/Pause");
    let rew_button = Button::new_with_label("Rewind");
    let ff_button = Button::new_with_label("Fast Forward");
    
    let image=Image::new_from_file(filename);
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
