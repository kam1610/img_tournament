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
use crate::get_month_img_files;
////////////////////////////////////////////////////////////
glib::wrapper! {
    pub struct ImtrButtonBox(ObjectSubclass<imp::ImtrButtonBox>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}
impl ImtrButtonBox{
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

                       file_dialog.select_folder(
                           Some(root_win),
                           None::<&Cancellable>,
                           clone!(@strong s, @strong lbl => move|r|{
                               let d = if let Ok(d) = r { d } else { return; };
                               println!("selected dir is {:?}", d.path());
                               lbl.set_text(d.path().unwrap().to_str().unwrap());
                               *s.imp().dir.borrow_mut() = d.path().unwrap();
                           }));
                   }));
    }
    // gen_btn_setup ///////////////////////////////////////
    fn gen_btn_setup(&self){
        let btn = self.imp().gen_btn.clone();

        let s = self.clone();
        btn.connect_clicked(
            clone!(@strong s => move|r|{
                let lst = get_month_img_files(&s.imp().dir.borrow(),
                                              s.imp().year_lbl.label().parse().unwrap(),
                                              s.imp().mon_btn.value() as u32);

            }));



        // let evt = ImtrEventObject::new();
        // evt.set_path(Path::new("/dev/shm/test.png"), Path::new("/dev/shm/test.png"),);
        // s.imp().mediator.borrow().as_ref().unwrap()
        //     .emit_by_name::<()>("directory-selected", &[&evt]);


    }
    // new /////////////////////////////////////////////////
    pub fn new() -> Self{
        let obj: ImtrButtonBox = Object::builder().build();
        obj.append(&obj.imp().dir_btn);
        obj.append(&obj.imp().dir_lbl);
        obj.dir_btn_setup();

        obj.append(&obj.imp().year_lbl);

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
