extern crate gtk;
use gtk::{ScrolledWindow,ContainerExt,Box,Orientation,Entry,Paned,Button,EntryExt,Frame};
use std::vec::Vec;

pub trait Attachable{
    fn attach<F:ContainerExt>(self,parent: &F);
}

pub struct Control {}

impl Control {
    pub fn play(&self){
        println!("Play")
    }

    pub fn rew(&self){
        println!("Rewind")
    }

    pub fn ff(&self){
        println!("Fast Forward")
    }
}
impl Clone for Control{
    fn clone(&self)->Self{
        return Control{}
    }
}
impl Copy for Control {}

pub struct LabelFrame {
    label_box:Box,
    label_pane:Paned,
    labels:Vec<Label>
}
impl LabelFrame{

    pub fn new()->LabelFrame{
        let pane=Paned::new(Orientation::Vertical);
        let label_box=Box::new(Orientation::Vertical,0);
        let test_label=Label::new();
        test_label.attach(&label_box);
        pane.add(&label_box);
        let add_button=Button::new_with_label("+");
        pane.add(&add_button);
        LabelFrame{label_box: label_box,label_pane:pane,labels:Vec::new()}
    }
}
impl Attachable for LabelFrame{
    fn attach<F:ContainerExt>(self,parent: &F){
        let scroll=ScrolledWindow::new(None,None);
        scroll.add(&self.label_pane);
        parent.add(&scroll);
    }
}


struct Label{
    key:Entry,
    value:Entry
}
impl Label{
    fn new()->Label{
        let k = Entry::new();
        let v = Entry::new();
        k.set_text("Label");
        v.set_text("Value");
        Label{key:k,value:v}
    }
}
impl Attachable for Label{
    fn attach<F:ContainerExt>(self,parent: &F){
        let f=Frame::new(None);
        let b=Box::new(Orientation::Vertical,0);
        b.add(&self.key);
        b.add(&self.value);
        f.add(&b);
        parent.add(&f);
    }
}
