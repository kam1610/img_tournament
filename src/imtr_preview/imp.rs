use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::cell::RefCell;
use std::cell::Cell;

use gtk::glib;
use gtk::glib::Object;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::subclass::prelude::*;
use gtk::glib::subclass::Signal;
use gtk::prelude::*;
use gtk::glib::Properties;

use crate::imtr_event_object::ImtrEventObject;
use crate::tree_util::Decision;
use crate::scale_factor::ScaleFactor;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DivState { N, H, V }

// struct //////////////////////////////////////////////////
#[derive(Debug, Properties)]
#[properties(wrapper_type = super::ImtrPreview)]
pub struct ImtrPreview{
    #[property(get, set)]
    pub(super) path_a       : RefCell<Option<PathBuf>>,
    pub(super) pbuf_a       : RefCell<Option<Pixbuf>>,
    pub(super) scale_pbuf_a : RefCell<Option<Pixbuf>>,
    pub(super) scale_fact_a : RefCell<ScaleFactor>,
    #[property(get, set)]
    pub(super) path_b       : RefCell<Option<PathBuf>>,
    pub(super) pbuf_b       : RefCell<Option<Pixbuf>>,
    pub(super) scale_pbuf_b : RefCell<Option<Pixbuf>>,
    pub(super) scale_fact_b : RefCell<ScaleFactor>,
    pub(super) divstate     : Cell<DivState>,
    #[property(get, set)]
    pub(super) mediator     : RefCell<Object>,
    #[property(get, set, builder(Decision::Undef))]
    pub(super) decision: Cell<Decision>,
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
    // properties //////////////////////////////////////////
    fn properties() -> &'static [glib::ParamSpec] {
        Self::derived_properties()
    }
    fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }
    fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        self.derived_property(id, pspec)
    }
}
impl WidgetImpl      for ImtrPreview {}
impl DrawingAreaImpl for ImtrPreview {}
impl Default         for ImtrPreview {
    fn default() -> Self{
        Self{
            path_a       : RefCell::new(None),
            pbuf_a       : RefCell::new(None),
            scale_pbuf_a : RefCell::new(None),
            scale_fact_a : RefCell::new(ScaleFactor{scale:0.0, dst_w:0, dst_h:0, ofst_x:0, ofst_y:0}),
            path_b       : RefCell::new(None),
            pbuf_b       : RefCell::new(None),
            scale_pbuf_b : RefCell::new(None),
            scale_fact_b : RefCell::new(ScaleFactor{scale:0.0, dst_w:0, dst_h:0, ofst_x:0, ofst_y:0}),
            divstate     : Cell::new(DivState::N),
            mediator     : RefCell::new(Object::with_type(glib::types::Type::OBJECT)),
            decision     : Cell::new(Decision::Undef),
        }
    }
}
