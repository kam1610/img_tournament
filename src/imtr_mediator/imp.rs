use std::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;
use once_cell::sync::Lazy;

use gtk::glib;
use gtk::glib::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib::object::Object;
use gtk::glib::subclass::Signal;
use gtk::glib::Properties;

use crate::imtr_event_object::ImtrEventObject;
use crate::tree_util::*;
////////////////////////////////////////////////////////////
#[derive(Debug, Properties)]
#[properties(wrapper_type = super::ImtrMediator)]
pub struct ImtrMediator{
    #[property(get, set)]
    pub(super) win        : RefCell<Object>,
    #[property(get, set)]
    pub(super) btn_box    : RefCell<Object>,
    #[property(get, set)]
    pub(super) pwin       : RefCell<Object>,
    pub(super) match_list : RefCell<Vec<Rc<RefCell<Node>>>>,
    pub(super) match_num  : Cell<usize>,
}
// subclass ////////////////////////////////////////////////
#[glib::object_subclass]
impl ObjectSubclass for ImtrMediator {
    const NAME: &'static str = "ImtrMediator";
    type Type = super::ImtrMediator;
    type ParentType = glib::Object;
}
// ObjectImpl //////////////////////////////////////////////
impl ObjectImpl for ImtrMediator{
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
    // signal //////////////////////////////////////////////
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("build-tournament")
                    .param_types([ImtrEventObject::static_type()])
                    .build(),
                Signal::builder("directory-selected")
                    .param_types([ImtrEventObject::static_type()])
                    .build(),
            ]
        });
        return SIGNALS.as_ref();
    }
}
// Default /////////////////////////////////////////////////
impl Default for ImtrMediator{
    fn default() -> Self{
        Self{
            win       : RefCell::new(Object::with_type(glib::types::Type::OBJECT)),
            pwin      : RefCell::new(Object::with_type(glib::types::Type::OBJECT)),
            btn_box   : RefCell::new(Object::with_type(glib::types::Type::OBJECT)),
            match_list: RefCell::new(vec![]),
            match_num : Cell::new(0),
        }
    }
}
