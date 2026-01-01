use once_cell::sync::Lazy;
use std::path::Path;
use std::path::PathBuf;
use std::cell::RefCell;
use std::cell::Cell;

use gtk::glib;
use gtk::glib::Object;
use gtk::DrawingArea;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::subclass::prelude::*;
use gtk::glib::subclass::Signal;
use gtk::prelude::*;

use crate::imtr_event_object::ImtrEventObject;
#[derive(Clone, Copy, PartialEq)]
pub enum DivState { N, H, V }
pub enum Decision { Undef, Left, Right }

// ScaleFactor /////////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct ScaleFactor{
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
// struct //////////////////////////////////////////////////
pub struct ImtrPreview{
    pub(super) path_a       : Option<PathBuf>,
    pub(super) pbuf_a       : RefCell<Option<Pixbuf>>,
    pub(super) scale_pbuf_a : RefCell<Option<Pixbuf>>,
    pub(super) scale_fact_a : RefCell<ScaleFactor>,
    pub(super) path_b       : Option<PathBuf>,
    pub(super) pbuf_b       : RefCell<Option<Pixbuf>>,
    pub(super) scale_pbuf_b : RefCell<Option<Pixbuf>>,
    pub(super) scale_fact_b : RefCell<ScaleFactor>,
    pub(super) decision     : Decision,
    pub(super) divstate     : Cell<DivState>,
}
// subclass ////////////////////////////////////////////////
#[glib::object_subclass]
impl ObjectSubclass for ImtrPreview {
    const NAME: &'static str = "ImtrPreview";
    type Type       = super::ImtrPreview;
    type ParentType = gtk::DrawingArea;
}
// GObject /////////////////////////////////////////////////
impl ObjectImpl for ImtrPreview {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("set-images")
                .param_types([ImtrEventObject::static_type()])
                .build(),
                 ]
        });
        return SIGNALS.as_ref();
    }
}
impl WidgetImpl      for ImtrPreview {}
impl DrawingAreaImpl for ImtrPreview {}
impl Default         for ImtrPreview {
    fn default() -> Self{
        Self{
            path_a       : None,
            pbuf_a       : RefCell::new(None),
            scale_pbuf_a : RefCell::new(None),
            scale_fact_a : RefCell::new(ScaleFactor{scale:0.0, dst_w:0, dst_h:0, ofst_x:0, ofst_y:0}),
            path_b       : None,
            pbuf_b       : RefCell::new(None),
            scale_pbuf_b : RefCell::new(None),
            scale_fact_b : RefCell::new(ScaleFactor{scale:0.0, dst_w:0, dst_h:0, ofst_x:0, ofst_y:0}),
            decision     : Decision::Undef,
            divstate     : Cell::new(DivState::N),
        }
    }
}
