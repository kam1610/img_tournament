use once_cell::sync::Lazy;

use gtk::glib;
use gtk::glib::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib::object::Object;
use gtk::glib::subclass::Signal;

use crate::imtr_event_object::ImtrEventObject;
////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct ImtrMediator{
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
    // signal //////////////////////////////////////////////
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
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
        Self{}
    }
}
