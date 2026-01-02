mod imp;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use std::fs::OpenOptions;

use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib;
use gtk::glib::object::Object;
use gtk::glib::closure_local;
use gtk::AlertDialog;
use gtk::Window;
use gtk::Orientation;
use gtk::Label;
use gtk::Button;
use gtk::glib::clone;
use gtk::gdk_pixbuf::InterpType;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::cairo::ImageSurface;
use gtk::cairo::Format;

use crate::imtr_event_object::ImtrEventObject;
use crate::imtr_button_box::ImtrButtonBox;
use crate::month_img_list::get_month_img_files;
use crate::imtr_preview::ImtrPreview;
use crate::tree_util::*;
use crate::scale_factor::ScaleFactor;
// wrapper /////////////////////////////////////////////////
glib::wrapper! {
    pub struct ImtrMediator(ObjectSubclass<imp::ImtrMediator>);
}
// ImtrMediator ////////////////////////////////////////////
impl ImtrMediator{

    // export-tournament-result ////////////////////////////
    fn export_tournament_result_sub(node: &Rc<RefCell<Node>>, depth: i32, width: i32,
                                    cr: &cairo::Context, img_px: i32, img_mgn: i32) -> i32{
        let ofx = depth * (img_px + img_mgn);
        let ofy = width * (img_px + img_mgn);
        if node.borrow().decision.get() != Decision::Undef{
            let p    = resolve_winner_leaf(node).unwrap();
            let pbuf = Pixbuf::from_file(p).expect("(export_tournament_result_sub) load img file error");
            let sfac = ScaleFactor::get_scale_offset(pbuf.width(), pbuf.height(), img_px, img_px);
            let sbuf = pbuf.scale_simple(sfac.dst_w, sfac.dst_h,InterpType::Bilinear).unwrap();
            let ofx  = ofx + sfac.ofst_x;
            let ofy  = ofy + sfac.ofst_y;

            cr.set_source_pixbuf(&sbuf, ofx as f64, ofy as f64);
            let extents = cr.clip_extents().unwrap();
            let width   = extents.2 - extents.0;  // x2 - x1
            let height  = extents.3 - extents.1; // y2 - y1
            cr.rectangle(0.0, 0.0, width, height);
            if cr.fill().is_err(){println!("draw image on PreviewWindow failed!"); }

        }
        0
    }
    fn export_tournament_result(&self){

        let list = &self.imp().match_list;
        let root = list.borrow().last().unwrap().clone();
        let img_px  = 80;
        let img_mgn =  4;

        let node_num = list.borrow().len();
        let leaf_num = node_num + 1;
        let target_h = (img_px + img_mgn) * (node_num + leaf_num) as i32;
        let target_w = (img_px + img_mgn) * ((node_num + leaf_num) as f64).log2().ceil() as i32;

        // 0. prepare surface
        let surface = {
            if let Ok(sf) = ImageSurface::create(Format::ARgb32, target_w, target_h) { sf }
            else { println!("(export_tounament_result) creating surface failed"); return; } };
        let cr = {
            if let Ok(ctx) = cairo::Context::new(&surface) { ctx }
            else { println!("(export_tounament_result) creating context failed"); return; } };

        Self::export_tournament_result_sub(&root, 0, 0, &cr, img_px, img_mgn);

        let mut path_buf = PathBuf::from("./img_tournament_result.png");
        let mut out_file  = {
            if let Ok(f) = OpenOptions::new()
                .read(false).write(true).create(true).open(&path_buf) { f }
            else { println!("(export_images) can not open: {}",
                            path_buf.to_str().unwrap()); return; } };
        surface.write_to_png(&mut out_file).expect("write_to_png in export_images");
    }
    // new /////////////////////////////////////////////////
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
                                              btn_box.imp().mon_btn.value() as u32,
                                              btn_box.get_filter_enabled())
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

                let match_list = Node::get_match_up_list( root.as_ref().expect("root node").clone() );

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
                let pwin_temp = s.imp().pwin.borrow();
                let pwin = pwin_temp.downcast_ref::<ImtrPreview>()
                    .expect("ImtrPreview is expected");
                let dec = pwin.property::<Decision>("decision");
                let ix  = s.imp().match_num.get();
                let sz  = s.imp().match_list.borrow().len();
                let win = s.imp().win.borrow().clone()
                    .downcast::<Window>().expect("Window");

                if dec == Decision::Undef {
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
                    return;
                }

                // winner
                let p = pwin.get_path(dec);

                let winner_win = Window::builder().title( String::from("The winner has been selected") )
                    .modal(true).build();
                let vbox          = gtk::Box::builder().orientation(Orientation::Vertical).build();
                let label_1       = Label::new(Some( &format!("the winner is {:?}", p.clone().unwrap() ) ));
                let label_2       = Label::new(Some("copy the path to the clipboar?"));
                let button_box    = gtk::Box::builder().orientation(Orientation::Horizontal).build();
                let ok_button     = Button::with_label("OK");
                let cancel_button = Button::with_label("Cancel");

                ok_button.connect_clicked(
                    clone!(@strong winner_win, @strong p => move|_b|{
                        let dp     = gtk::gdk::Display::default().unwrap();
                        let cb     = dp.clipboard();
                        let p      = p.as_ref().unwrap();
                        cb.set_text(&p.to_str().unwrap());
                        winner_win.close(); }));

                cancel_button.connect_clicked(
                    clone!(@strong winner_win => move|_b|{ winner_win.close(); }));

                button_box.append(&ok_button);
                button_box.append(&cancel_button);

                vbox.append(&label_1);
                vbox.append(&label_2);
                vbox.append(&button_box);

                winner_win.set_child(Some(&vbox));
                winner_win.present();

                // experimental!
                s.export_tournament_result();

                return;

            })
        );
        // prev-match //////////////////////////////////////
        obj.connect_closure(
            "prev-match",
            false,
            closure_local!(|s: Self, e: ImtrEventObject|{
                let pwin_temp = s.imp().pwin.borrow();
                let pwin = pwin_temp.downcast_ref::<ImtrPreview>()
                    .expect("ImtrPreview is expected");
                let ix  = s.imp().match_num.get();
                let win = s.imp().win.borrow().clone()
                    .downcast::<Window>().expect("Window");

                if 0 < ix{
                    let n_temp = s.imp().match_list.borrow();
                    let n = n_temp[ix-1].borrow();

                    s.imp().match_num.set(ix-1);

                    let path_l = resolve_winner_leaf(&n.left.as_ref().unwrap() );
                    let path_r = resolve_winner_leaf(&n.right.as_ref().unwrap());

                    let evt = ImtrEventObject::new();
                    evt.set_path(path_l, path_r);
                    s.imp().pwin.borrow().clone()
                        .downcast::<ImtrPreview>()
                        .expect("imtr_preview is expected")
                        .emit_by_name::<()>("set-images", &[&evt]);
                    return;
                }
            }));
        return obj;
    }
}
