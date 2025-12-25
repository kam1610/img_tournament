mod imp;

use std::path::Path;
use std::path::PathBuf;

use gtk::prelude::*;
use gtk::Accessible;
use gtk::Buildable;
use gtk::ConstraintTarget;
use gtk::glib;
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::glib::Object;
use gtk::glib::closure_local;
use gtk::gdk_pixbuf::InterpType;
use gtk::gdk_pixbuf::Pixbuf;

use crate::imtr_event_object::ImtrEventObject;

// wrapper /////////////////////////////////////////////////
glib::wrapper! {
    pub struct ImtrPreview(ObjectSubclass<imp::ImtrPreview>)
        @extends gtk::DrawingArea, gtk::Widget,
        @implements Accessible, Buildable, ConstraintTarget;
}
// get_scale_offset ////////////////////////////////////////
struct ScaleFactor{
    pub scale  : f64,
    pub dst_w  : i32, pub dst_h  : i32,
    pub ofst_x : i32, pub ofst_y : i32,
}
impl ScaleFactor{
    pub fn get_scale_offset(src_w: i32, src_h: i32, dst_w: i32, dst_h: i32 ) -> Self{
        let scale = (dst_w as f64) / (src_w as f64);
        if ((src_h as f64) * scale) <= (dst_h as f64) {
            let ofst_y = (((dst_h as f64) - ((src_h as f64) * scale)) / 2.0) as i32;
            return Self{
                scale: scale,
                dst_w: dst_w,
                dst_h: ((src_h as f64) * scale) as i32,
                ofst_x: 0, ofst_y: ofst_y};
        }
        let scale = (dst_h as f64) / (src_h as f64);
        let ofst_x = (((dst_w as f64) - ((src_w as f64) * scale)) / 2.0) as i32;
        return Self{
            scale: scale,
            dst_w: ((src_w as f64) * scale) as i32,
            dst_h: dst_h,
            ofst_x: ofst_x, ofst_y: 0
        };
    }
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
    fn prepare_scale_buf(&self){
        // とりあえず両方ある前提
        let scale_buf_a;
        let scale_buf_b;
        {
            let pbuf_a_opt = self.imp().pbuf_a.borrow();
            let pbuf_b_opt = self.imp().pbuf_b.borrow();
            let pbuf_a     = pbuf_a_opt.as_ref().unwrap();
            let pbuf_b     = pbuf_b_opt.as_ref().unwrap();

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
                    (result_h_a.dst_w,  result_h_a.dst_h, result_h_b.dst_w, result_h_b.dst_w)
                } else {
                    (result_v_a.dst_w,  result_v_a.dst_h, result_v_b.dst_w, result_v_b.dst_w)
                };
            scale_buf_a = pbuf_a.scale_simple(a_w, a_h, InterpType::Bilinear).unwrap();
            scale_buf_b = pbuf_b.scale_simple(b_w, b_h, InterpType::Bilinear).unwrap();
        }
        *self.imp().pbuf_a.borrow_mut() = Some(scale_buf_a);
        *self.imp().pbuf_b.borrow_mut() = Some(scale_buf_b);
    }
    // update_pixbuf ///////////////////////////////////////
    fn update_pixbuf(p: Self, e: ImtrEventObject){
        let (path_a, path_b) = e.get_path();
        p.set_buf_from_path(path_a, path_b);
        p.prepare_scale_buf();
        p.queue_draw();
    }
    // new /////////////////////////////////////////////////
    pub fn new() -> Self{
        let obj : ImtrPreview = Object::builder().build();
        obj.set_hexpand(true);
        obj.set_vexpand(true);

        /*

        参考: 描画先ウィンドウサイズは obj.set_draw_func の内部で下記のようにpwin.width()などで参照している

                    // scale from target to pwin
            let (tgt_to_pwin_scale, _, _, _, _) =
               util::get_scale_offset(target_w, target_h, pwin.width(), pwin.height());

         */


        obj.connect_closure(
            "set-images",
            false,
            closure_local!(|p: Self, e: ImtrEventObject|{

            })
        );
        return obj;
    }
}
