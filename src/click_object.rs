extern crate gtk;
use gtk::{ScrolledWindow,ContainerExt,Box,Orientation,Entry,EntryExt,Frame};
use std::vec::Vec;

pub trait Attachable{
    fn attach<F:ContainerExt>(&self,parent: &F);
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
    labels:Vec<Label>
}
impl LabelFrame{

    pub fn new(labels:Vec<Label>)->LabelFrame{
        let label_box=Box::new(Orientation::Vertical,0);
        //Add labels here
        for label in &labels{
            label.attach(&label_box)
        }
        LabelFrame{label_box: label_box,labels:labels}
    }
}
impl Attachable for LabelFrame{
    fn attach<F:ContainerExt>(&self,parent: &F){
        let scroll=ScrolledWindow::new(None,None);
        scroll.add(&self.label_box);
        parent.add(&scroll);
    }
}

pub struct Label{
    key:String,
    value:Entry
}
impl Label{
    pub fn new(label:&str)->Label{
        let k = String::from(label);
        let v = Entry::new();
        v.set_text("0");
        Label{key:k,value:v}
    }
}
impl Attachable for Label{
    fn attach<F:ContainerExt>(&self,parent: &F){
        let f=Frame::new(Some(self.key.as_str()));
        f.add(&self.value);
        parent.add(&f);
    }
}
