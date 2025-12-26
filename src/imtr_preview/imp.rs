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
pub enum DivState { N, H, V }
pub enum Decision { Undef, Left, Right }
// struct //////////////////////////////////////////////////
pub struct ImtrPreview{
    pub(super) path_a       : Option<PathBuf>,
    pub(super) pbuf_a       : RefCell<Option<Pixbuf>>,
    pub(super) scale_pbuf_a : RefCell<Option<Pixbuf>>,
    pub(super) path_b       : Option<PathBuf>,
    pub(super) pbuf_b       : RefCell<Option<Pixbuf>>,
    pub(super) scale_pbuf_b : RefCell<Option<Pixbuf>>,
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
            path_b       : None,
            pbuf_b       : RefCell::new(None),
            scale_pbuf_b : RefCell::new(None),
            decision     : Decision::Undef,
            divstate     : Cell::new(DivState::N),
        }
    }
}
