mod imp;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;

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
use crate::imtr_preview::ImtrPreview;
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
                        .build().show(Some(&win));
                    return;
                }

                let mut root: Option<Rc<RefCell<Node>>> = None;
                let mut opt = 1;
                for p in lst{
                    root = Some(insert(root.take(), p, opt)); opt+= 1; } // build tree
                print_tree(&root.clone().unwrap(), 0);

                let match_list = Node::get_match_up_list( root.expect("root node") );

                for m in match_list.iter(){
                    println!("opt: {}, h: {}, path: {:?}",
                             m.borrow().opt, m.borrow().depth, m.borrow().path);
                }

                *s.imp().match_list.borrow_mut() = match_list;
                s.imp().match_num.set(0);

                let c = s.imp().match_list.borrow()[0].clone();
                let c = c.borrow();
                let evt = ImtrEventObject::new();
                let pa: Option<PathBuf> =
                    if c.left.is_some() &&  c.left.as_ref().unwrap().borrow().path.is_some(){
                        (&c. left.as_ref().unwrap().borrow().path).clone()
                    } else {
                        None
                    };
                let pb: Option<PathBuf> =
                    if c.right.is_some() &&  c.right.as_ref().unwrap().borrow().path.is_some(){
                        (&c. right.as_ref().unwrap().borrow().path).clone()
                    } else {
                        None
                    };
                evt.set_path(pa, pb);
                s.imp().pwin.borrow().clone()
                    .downcast::<ImtrPreview>()
                    .expect("(ImtrMediator::build-tournament) imtr_preview is expected")
                    .emit_by_name::<()>("set-images", &[&evt]);

            })
        );
        // next-match //////////////////////////////////////
        obj.connect_closure(
            "next-match",
            false,
            closure_local!(|s: Self, e: ImtrEventObject|{
                let dec = s.imp().pwin.borrow().downcast_ref::<ImtrPreview>()
                    .expect("ImtrPreview is expected")
                    .property::<Decision>("decision");
                let ix = s.imp().match_num.get();
                let sz = s.imp().match_list.borrow().len();

                if dec == Decision::Undef {
                    let win = s.imp().win.borrow().clone()
                        .downcast::<Window>().expect("Window");
                    let alert = AlertDialog::builder()
                        .modal(true)
                        .message("please click one of the images")
                        .build().show(Some(&win));
                    return;
                }
                // update decision
                &s.imp().match_list.borrow()[ix].borrow_mut().decision.set(dec);
                // obtain next match
                if ix < (sz - 1){
                    let n_temp = s.imp().match_list.borrow();
                    let n = n_temp[ix+1].borrow();

                    s.imp().match_num.set(ix+1);

                    let path_l = resolve_winner_leaf(&n.left.as_ref().unwrap() );
                    let path_r = resolve_winner_leaf(&n.right.as_ref().unwrap());

                    let evt = ImtrEventObject::new();
                    evt.set_path(path_l, path_r);
                    s.imp().pwin.borrow().clone()
                        .downcast::<ImtrPreview>()
                        .expect("imtr_preview is expected")
                        .emit_by_name::<()>("set-images", &[&evt]);
                }
            })
        );
        return obj;
    }
}
