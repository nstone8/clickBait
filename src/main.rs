extern crate gtk;
extern crate regex;

use gtk::prelude::*;

mod click_object;
use click_object::{Control,LabelFrame,Attachable,Label};

use std::path::PathBuf;

use regex::Regex;

use gtk::{Button, WindowType, Image,Orientation,Box,ScrolledWindow,Paned,FileChooserDialog,DialogExt,FileChooserAction,ResponseType,Window,Dialog,TextView,ContainerExt,DialogFlags,WrapMode};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    //Choose labels
    let label_dialog_window=Window::new(WindowType::Toplevel);
    
    let label_dialog = Dialog::new_with_buttons(Some("Define Labels"),Some(&label_dialog_window),DialogFlags::empty(),&[("Done",ResponseType::Ok.into()),("Cancel",ResponseType::Cancel.into())]);
    let instructions=gtk::Label::new(Some("Enter label names separated by commas. All whitespace will be ignored."));
    let label_text = TextView::new();
    label_text.set_wrap_mode(WrapMode::Word);
    let label_scroll = ScrolledWindow::new(None,None);
    label_scroll.add(&label_text);
    label_dialog.get_content_area().add(&instructions);  
    label_dialog.get_content_area().add(&label_scroll);
    instructions.show();
    label_scroll.show_all();
    let label_response=label_dialog.run();
    let label_vec_option:Option<Vec<Label>>;
    if label_response==ResponseType::Ok.into(){
    let label_str:String;
        match label_text.get_buffer().unwrap().get_bounds(){
            (begin,end) => label_str = label_text.get_buffer().unwrap().get_text(&begin,&end,false).unwrap()
        }
        println!("Entered Text:{}",label_str);
        let label_match=Regex::new(r"(.*?)(,|$)").unwrap();
        let label_iter=label_match.captures_iter(label_str.trim());
        let mut label_vec: Vec<Label> = Vec::new();
        for label in label_iter{
            let l=label.iter().nth(1).unwrap().unwrap().as_str().trim();
            println!("label: {}",l);
            label_vec.push(Label::new(l));
        }
        if label_vec.len()==0{
            label_vec_option=None;
        }else{
            label_vec_option=Some(label_vec);
        }
    }else{
        println!("No labels");
        label_vec_option=None;
    }
    label_dialog.destroy();
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
    if let Some(all_labels)=label_vec_option{
        let labels=LabelFrame::new(all_labels);
        labels.attach(&top_box);
    }
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
