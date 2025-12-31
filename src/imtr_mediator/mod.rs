mod imp;

use std::rc::Rc;
use std::cell::RefCell;

use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib;
use gtk::glib::object::Object;
use gtk::glib::closure_local;
use gtk::AlertDialog;
use gtk::Window;

use crate::imtr_event_object::ImtrEventObject;
use crate::imtr_button_box::ImtrButtonBox;
use crate::month_img_list::get_month_img_files;
use crate::tree_util::*;
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
            closure_local!(|s: Self, e: ImtrEventObject|{
                println!("(ImtrMediator) directory-selected");
            })
        );
        // build-tournament ////////////////////////////////
        obj.connect_closure(
            "build-tournament",
            false,
            closure_local!(|s: Self, e: ImtrEventObject|{
                let btn_box = s.imp().btn_box.borrow().clone()
                    .downcast::<ImtrButtonBox>().expect("ImtrButtonBox");
                let lst = get_month_img_files(&btn_box.imp().dir.borrow(),
                                              btn_box.imp().year_btn.value() as i32,
                                              btn_box.imp().mon_btn.value() as u32)
                    .expect("tournament list");
                if lst.len() < 2 {
                    let win = s.imp().win.borrow().clone()
                        .downcast::<Window>().expect("Window");
                    let alert = AlertDialog::builder()
                        .modal(true)
                        .message("please choose directory contains at least 2 image files")
                        .build()
                        .show(Some(&win));
                    return;
                }

                let mut root: Option<Rc<RefCell<Node>>> = None;
                for p in lst{
                    root = Some(insert(root.take(), p)); } // build tree
                print_tree(&root.clone().unwrap(), 0);

                let c = next_candidate(&root.clone().unwrap());
                println!("next candidate {:?}", c);
                // todo
                // 最初の対戦を取り出したのち
                // preview_panel にパスお設定する

            })
        );

        return obj;
    }
}
