use std::cell::Cell;
use std::cell::RefCell;
use std::path::PathBuf;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use glib::ParamSpec;
use glib::Properties;
use glib::Value;

// ImtrEventObject /////////////////////////////////////////
#[derive(Properties)]
#[properties(wrapper_type = super::ImtrEventObject)]
pub struct ImtrEventObject{
    // todo: eventtype and args
    #[property(get, set)]
    pub id: Cell<i32>,
    pub path_a  : RefCell<Option<PathBuf>>,
    pub path_b  : RefCell<Option<PathBuf>>,
}

// Subclass ////////////////////////////////////////////////
#[glib::object_subclass]
impl ObjectSubclass for ImtrEventObject{
    const NAME: &'static str = "ImtrEventObject";
    type Type = super::ImtrEventObject;
}
// GObject /////////////////////////////////////////////////
impl ObjectImpl for ImtrEventObject{
    fn properties() -> &'static [ParamSpec] {
        Self::derived_properties()
    }
    fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }
    fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
        self.derived_property(id, pspec)
    }
}
// default /////////////////////////////////////////////////
impl Default for ImtrEventObject{
    fn default() -> Self{
        Self{
            id : Cell::new(0),
            path_a : RefCell::new(None),
            path_b : RefCell::new(None),
        }
    }
}
