mod imp;

use std::path::Path;
use std::path::PathBuf;
use std::cell::RefCell;
use std::cell::Cell;

use gtk::prelude::*;
use gtk::DrawingArea;
use gtk::Accessible;
use gtk::Buildable;
use gtk::ConstraintTarget;
use gtk::glib;
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::glib::Object;
use gtk::glib::closure_local;
use gtk::gdk_pixbuf::InterpType;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::GestureClick;

use crate::imtr_event_object::ImtrEventObject;
use crate::imtr_preview::imp::{DivState};
use crate::imtr_mediator::ImtrMediator;
use crate::tree_util::Decision;
use imp::ScaleFactor;

// wrapper /////////////////////////////////////////////////
glib::wrapper! {
    pub struct ImtrPreview(ObjectSubclass<imp::ImtrPreview>)
        @extends gtk::DrawingArea, gtk::Widget,
        @implements Accessible, Buildable, ConstraintTarget;
}
// impl ////////////////////////////////////////////////////
impl ImtrPreview{
    // set_buf_from_path ///////////////////////////////////
    fn set_buf_from_path(&self, path_a: PathBuf, path_b: PathBuf) {
        println!("(PreviewWindow) loading new pixbuf file: {}, {}",
                 path_a.to_str().unwrap(), path_b.to_str().unwrap());
        *self.imp().pbuf_a.borrow_mut() = if let Ok(p) = Pixbuf::from_file( &path_a ){
            Some(p) } else { None };
        *self.imp().pbuf_b.borrow_mut() = if let Ok(p) = Pixbuf::from_file( &path_b ){
            Some(p) } else { None };
    }
    // prepare_scale_buf ///////////////////////////////////
    fn prepare_scale_buf_sub(&self,
                             pbuf      : RefCell<Option<Pixbuf>>,
                             scale_pbuf: RefCell<Option<Pixbuf>>,
                             scale_fact: RefCell<ScaleFactor>){

        let pbuf_temp_opt   = pbuf.borrow();
        let pbuf_temp       = pbuf_temp_opt.as_ref().unwrap();
        let scale_fact_temp = ScaleFactor::get_scale_offset(pbuf_temp.width(), pbuf_temp.height(),
                                                            self.width(), self.height() );
        let scale_pbuf_temp = pbuf_temp.scale_simple(scale_fact_temp.dst_w, scale_fact_temp.dst_h,
                                                     InterpType::Bilinear).unwrap();

        *scale_fact.borrow_mut() = scale_fact_temp;
        *scale_pbuf.borrow_mut()  = Some(scale_pbuf_temp);
        self.imp().divstate.set(DivState::N);
    }
    fn prepare_scale_buf(&self){
        if self.imp().pbuf_a.borrow().is_some() && self.imp().pbuf_b.borrow().is_none() { // onlyA
            self.prepare_scale_buf_sub(self.imp().pbuf_a.clone(),
                                       self.imp().scale_pbuf_a.clone(),
                                       self.imp().scale_fact_a.clone());
        } else if self.imp().pbuf_a.borrow().is_none() && self.imp().pbuf_b.borrow().is_some() { // onlyB
            self.prepare_scale_buf_sub(self.imp().pbuf_b.clone(),
                                       self.imp().scale_pbuf_b.clone(),
                                       self.imp().scale_fact_b.clone());
        } else if self.imp().pbuf_a.borrow().is_none() && self.imp().pbuf_b.borrow().is_none(){ //noneBoth
            self.imp().divstate.set(DivState::N);
        } else { // both exist
            let scale_buf_a;
            let scale_buf_b;
            {
                let pbuf_a_opt = self.imp().pbuf_a.borrow();
                let pbuf_b_opt = self.imp().pbuf_b.borrow();
                let pbuf_a     = pbuf_a_opt.as_ref().unwrap();
                let pbuf_b     = pbuf_b_opt.as_ref().unwrap();

                if (self.width() == 0) || (self.height() == 0) {
                    self.imp().divstate.set(DivState::N);
                    return;
                }

                println!("draw area size: {}, {}", self.width(), self.height());

                // try div horizontal
                let result_h_a = ScaleFactor::get_scale_offset(pbuf_a.width(), pbuf_a.height(),
                                                               self.width(), self.height()/2 );
                let result_h_b = ScaleFactor::get_scale_offset(pbuf_b.width(), pbuf_b.height(),
                                                               self.width(), self.height()/2 );
                let area_h =
                    (result_h_a.scale * pbuf_a.width() as f64 * result_h_a.scale * pbuf_a.height() as f64) +
                    (result_h_b.scale * pbuf_b.width() as f64 * result_h_b.scale * pbuf_b.height() as f64);
                // try div vertical
                let result_v_a = ScaleFactor::get_scale_offset(pbuf_a.width(), pbuf_a.height(),
                                                               self.width()/2, self.height() );
                let result_v_b = ScaleFactor::get_scale_offset(pbuf_b.width(), pbuf_b.height(),
                                                               self.width()/2, self.height() );
                let area_v =
                    (result_v_a.scale * pbuf_a.width() as f64 * result_v_a.scale * pbuf_a.height() as f64) +
                    (result_v_b.scale * pbuf_b.width() as f64 * result_v_b.scale * pbuf_b.height() as f64);
                // compare area size
                let (a_w, a_h, b_w, b_h) =
                    if area_v < area_h {
                        self.imp().divstate.set(DivState::H);
                        *self.imp().scale_fact_a.borrow_mut() = result_h_a.clone();
                        *self.imp().scale_fact_b.borrow_mut() = result_h_b.clone();
                        (result_h_a.dst_w,  result_h_a.dst_h, result_h_b.dst_w, result_h_b.dst_h)
                    } else {
                        self.imp().divstate.set(DivState::V);
                        *self.imp().scale_fact_a.borrow_mut() = result_v_a.clone();
                        *self.imp().scale_fact_b.borrow_mut() = result_v_b.clone();
                        (result_v_a.dst_w,  result_v_a.dst_h, result_v_b.dst_w, result_v_b.dst_h)
                    };
                println!("calculated: {}, {}, {}, {}\n {:?}\n {:?}\n {:?}\n {:?}",
                         a_w, a_h, b_w, b_h, result_h_a, result_h_b, result_v_a, result_v_b);
                scale_buf_a = pbuf_a.scale_simple(a_w, a_h, InterpType::Bilinear).unwrap();
                scale_buf_b = pbuf_b.scale_simple(b_w, b_h, InterpType::Bilinear).unwrap();
            }
            *self.imp().scale_pbuf_a.borrow_mut() = Some(scale_buf_a);
            *self.imp().scale_pbuf_b.borrow_mut() = Some(scale_buf_b);
        }
    }
    // update_pixbuf ///////////////////////////////////////
    fn update_pixbuf(&self, e: ImtrEventObject){
        let (path_a, path_b) = e.get_path();
        // self.imp().path_a = Some(path_a);
        // self.imp().path_b = Some(path_b);
        self.imp().decision.set(Decision::Undef);
        self.set_buf_from_path(path_a, path_b);
        self.prepare_scale_buf();
        self.queue_draw();
    }
    // draw_func ///////////////////////////////////////////
    fn draw_func_sub(&self,
                     cr: &cairo::Context, spbuf: RefCell<Option<Pixbuf>>, sf: &RefCell<ScaleFactor>, ofst: bool){
        if spbuf.borrow().is_some() {
            let p = spbuf.borrow();
            let scale_crop_pixbuf = {
                if let Some(ref p) = p.as_ref() { p.clone() }
                else { return; }};

            let mut x = 0.0; let mut y = 0.0;
            if ofst {
                if self.imp().divstate.get() == DivState::H { y = self.height() as f64 / 2.0; }
                if self.imp().divstate.get() == DivState::V { x = self.width()  as f64 / 2.0; }
            }
            cr.set_source_pixbuf(&scale_crop_pixbuf,
                                 sf.borrow().ofst_x as f64 + x,
                                 sf.borrow().ofst_y as f64 + y);
            cr.rectangle(0.0, 0.0,
                         self.width() as f64, self.height() as f64);
            if cr.fill().is_err(){
                println!("draw image on PreviewWindow failed!"); }
        }
    }
    fn draw_func(da: &DrawingArea, cr: &cairo::Context, w: i32, h: i32){
        let pwin = da.clone().downcast::<ImtrPreview>().expect("imtr_preview");
        pwin.draw_func_sub(cr, pwin.imp().scale_pbuf_a.clone(), &pwin.imp().scale_fact_a, false);
        pwin.draw_func_sub(cr, pwin.imp().scale_pbuf_b.clone(), &pwin.imp().scale_fact_b, true );

        // selection frame
        if pwin.imp().decision.get() == Decision::Undef { return; }
        cr.set_source_rgb(0.0, 0.0, 1.0);
        cr.set_line_width(4.0);
        if pwin.imp().divstate.get() == DivState::H {
            if pwin.imp().decision.get() == Decision::Left { // upper half
                cr.rectangle(0.0,                 0.0,
                             pwin.width() as f64, (pwin.height() / 2) as f64 - 2.0);
                println!("upper half");
            } else { // lower half
                cr.rectangle(0.0,                 (pwin.height() / 2) as f64 + 2.0,
                             pwin.width() as f64, (pwin.height() / 2) as f64);
                println!("lower half");
            }
        } else {
            if pwin.imp().decision.get() == Decision::Left { //  left half
                cr.rectangle(0.0,                             0.0,
                             (pwin.width() / 2) as f64 - 2.0, pwin.height() as f64);
                println!("left half");
            } else { // right half
                cr.rectangle((pwin.width() / 2) as f64 + 2.0, 0.0,
                             (pwin.width() / 2) as f64,       pwin.height() as f64);
                println!("right half");
            }
        }
        cr.stroke();
    }
    // get_path ////////////////////////////////////////
    pub fn get_path(&self, dec: Decision) -> Option<PathBuf>{
        match dec{
            Decision::Left  => self.imp().path_a.clone(),
            Decision::Right => self.imp().path_b.clone(),
            Decision::Undef => None,
        }
    }
    // new /////////////////////////////////////////////////
    pub fn new() -> Self{
        let obj : ImtrPreview = Object::builder().build();
        obj.set_hexpand(true);
        obj.set_vexpand(true);
        obj.set_draw_func(Self::draw_func);
        obj.connect_resize( |da, _w, _h| {
            let pwin = da.clone().downcast::<ImtrPreview>().expect("imtr_preview");
            pwin.prepare_scale_buf();
            pwin.queue_draw();
        });
        // set-images //////////////////////////////////////
        obj.connect_closure(
            "set-images",
            false,
            closure_local!(|p: Self, e: ImtrEventObject|{
                p.update_pixbuf(e);
            })
        );
        // gesture/preview-clicked /////////////////////////
        let gesture_ctrl = GestureClick::new();
        gesture_ctrl.connect_released(|g,_n,x,y|{
            let pwin = g.widget()
                .downcast::<ImtrPreview>().expect("preview window is expect");
            let mediator = pwin.imp().mediator.clone().borrow().clone()
                .downcast::<ImtrMediator>().expect("imtr mediator is expected");
            if ((pwin.imp().divstate.get() == DivState::H) &&
                ((y as i32) < (pwin.height() / 2))) ||
                ((pwin.imp().divstate.get() == DivState::V) &&
                 ((x as i32) < (pwin.width() / 2)))  {
                     pwin.imp().decision.set(Decision::Left);
                    println!("left is selected");
            } else {
                    pwin.imp().decision.set(Decision::Right);
                    println!("right is selected");
            }
            pwin.queue_draw();
        });
        obj.add_controller(gesture_ctrl);
        return obj;
    }
}
