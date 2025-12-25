mod imp;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib;
use gtk::glib::object::Object;
use gtk::glib::closure_local;

use crate::imtr_event_object::ImtrEventObject;
use crate::month_img_list::get_month_img_files;
// wrapper /////////////////////////////////////////////////
glib::wrapper! {
    pub struct ImtrMediator(ObjectSubclass<imp::ImtrMediator>);
}
// ImtrMediator ////////////////////////////////////////////
impl ImtrMediator{
    pub fn new() -> Self{
        let obj = glib::Object::new::<ImtrMediator>();
        // directory-selected //////////////////////////////
        obj.connect_closure(
            "directory-selected",
            false,
            closure_local!(|m: Self, e: ImtrEventObject|{
                println!("(ImtrMediator) directory-selected");
            })
        );
        return obj;
    }
}
