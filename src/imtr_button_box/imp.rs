use std::cell::RefCell;
use std::cell::Cell;
use std::path::PathBuf;
use once_cell::sync::Lazy;

use gtk::Button;
use gtk::SpinButton;
use gtk::Adjustment;
use gtk::Label;
use gtk::subclass::prelude::*;
use gtk::glib;
use gtk::glib::subclass::Signal;
use gtk::glib::object::WeakRef;
use gtk::glib::Object;
// ImtrButtonBox ///////////////////////////////////////////
#[derive(Debug)]
pub struct ImtrButtonBox {
    pub(super) dir_btn  : Button,
    pub(super) dir_lbl  : Label,
    pub(super) year_lbl : Label,
    pub(super) mon_lbl  : Label,
    pub(super) mon_btn  : SpinButton,
    pub(super) gen_btn  : Button,
    pub(super) prev_btn : Button,
    pub(super) next_btn : Button,
    pub(super) save_btn : Button,
    pub(super) load_btn : Button,
    pub(super) mediator : RefCell<Option<Object>>,
    pub(super) dir      : RefCell<PathBuf>,
}
// GObject /////////////////////////////////////////////////
#[gtk::glib::object_subclass]
impl ObjectSubclass for ImtrButtonBox{
    const NAME: &'static str = "ImtrButtonBox";
    type Type = super::ImtrButtonBox;
    type ParentType = gtk::Box;
}
impl ObjectImpl for ImtrButtonBox{
    fn signals() -> &'static [Signal]{
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(||{
            vec![
            ]
        });
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for ImtrButtonBox {}
impl BoxImpl    for ImtrButtonBox {}
impl Default    for ImtrButtonBox {
    fn default() -> Self{
        let mod_adj = Adjustment::builder()
            .lower(1.0).upper(12.0)
            .step_increment(1.0).page_increment(1.0)
            .value(1.0)
            .build();
        let spin = SpinButton::builder()
            .adjustment(&mod_adj)
            .digits(0).wrap(true).width_request(2).visible(true)
            .build();
        Self{
            dir_btn  : Button::with_label("dir"),
            dir_lbl  : Label::builder().css_classes(vec!["dir_lbl"]).label("...").build(),
            year_lbl : Label::new(Some("year:2024")),
            mon_lbl  : Label::new(Some("mon:")),
            mon_btn  : spin,
            gen_btn  : Button::with_label("gen"),
            prev_btn : Button::with_label("prev"),
            next_btn : Button::with_label("next"),
            save_btn : Button::with_label("save"),
            load_btn : Button::with_label("load"),
            mediator : RefCell::new(None),
            dir      : RefCell::new(PathBuf::new()),
        }
    }
}
