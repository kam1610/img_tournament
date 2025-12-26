pub(crate) mod imp;

use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

use glib::Object;
use glib::subclass::types::ObjectSubclassIsExt;
use gtk::glib;
// wrapper /////////////////////////////////////////////////
glib::wrapper! {
    pub struct ImtrEventObject(ObjectSubclass<imp::ImtrEventObject>);
}
// impl ////////////////////////////////////////////////////
impl ImtrEventObject{
    pub fn new() -> Self{
        let obj : ImtrEventObject = Object::builder().build();
        return obj;
    }
    pub fn get_path(&self) -> (PathBuf, PathBuf){
        return (self.imp().path_a.borrow().clone().unwrap(),
                self.imp().path_b.borrow().clone().unwrap()); }
    pub fn set_path(&self, pa: &Path, pb: &Path){
        *self.imp().path_a.borrow_mut() = Some(pa.to_path_buf());
        *self.imp().path_b.borrow_mut() = Some(pb.to_path_buf());
    }
}
