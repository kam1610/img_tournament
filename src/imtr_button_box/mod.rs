mod imp;

use std::path::Path;

use gtk::gio::Cancellable;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::FileDialog;
use gtk::Window;
use gtk::Button;
use gtk::glib;
use gtk::Label;
use gtk::glib::object::Object;
use gtk::glib::clone;
use gtk::glib::WeakRef;

use crate::imtr_event_object::ImtrEventObject;
////////////////////////////////////////////////////////////
glib::wrapper! {
    pub struct ImtrButtonBox(ObjectSubclass<imp::ImtrButtonBox>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}
impl ImtrButtonBox{
    // label_update (for debug) ////////////////////////////
    fn label_update(lbl: &Label){ lbl.set_text("newlabel"); }
    // set_mediator ////////////////////////////////////////
    pub fn set_mediator(&self, m: Option<Object>){ *self.imp().mediator.borrow_mut() = m; }
    // dir_btn_setup ///////////////////////////////////////
    fn dir_btn_setup(&self){
        let btn = self.imp().dir_btn.clone();
        let lbl = self.imp().dir_lbl.clone();

        let file_dialog = FileDialog::builder().modal(true).build();
        let s = self.clone();
        btn.connect_clicked(
            clone!(@strong s,
                   @strong lbl => move |_b|{
                       let root_win = s.root().expect("root doesn't exist");
                       let root_win = root_win.downcast_ref::<Window>().unwrap();
                       let evt      = ImtrEventObject::new();
                       evt.set_path(Path::new("/dev/shm"));
                       s.imp().mediator.borrow().as_ref().unwrap()
                           .emit_by_name::<()>("directory-selected", &[&evt]);

                       file_dialog.open(
                           Some(root_win),
                           None::<&Cancellable>,
                           clone!(@strong s, @strong lbl => move|_r|{
                               // todo: notify mediator
                               lbl.set_text("clicked");
                           }));
                   }));
    }
    // gen_btn_setup ///////////////////////////////////////
    fn gen_btn_setup(&self){
        let btn = self.imp().gen_btn.clone();
    }
    // new /////////////////////////////////////////////////
    pub fn new() -> Self{
        let obj: ImtrButtonBox = Object::builder().build();
        obj.append(&obj.imp().dir_btn);
        obj.append(&obj.imp().dir_lbl);
        Self::label_update(&obj.imp().dir_lbl);
        obj.dir_btn_setup();

        obj.append(&obj.imp().mon_lbl);
        obj.append(&obj.imp().mon_btn);

        obj.gen_btn_setup();
        obj.append(&obj.imp().gen_btn);

        obj.append(&obj.imp().prev_btn);
        obj.append(&obj.imp().next_btn);
        obj.append(&obj.imp().save_btn);
        obj.append(&obj.imp().load_btn);

        return obj;
    }
}
