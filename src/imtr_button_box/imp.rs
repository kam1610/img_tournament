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
    pub dir_btn  : Button,
    pub dir_lbl  : Label,
    pub year_lbl : Label,
    pub year_btn : SpinButton,
    pub mon_lbl  : Label,
    pub mon_btn  : SpinButton,
    pub gen_btn  : Button,
    pub prev_btn : Button,
    pub next_btn : Button,
    pub save_btn : Button,
    pub load_btn : Button,
    pub mediator : RefCell<Option<Object>>,
    pub dir      : RefCell<PathBuf>,
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
        let mod_adj_m = Adjustment::builder()
            .lower(1.0).upper(12.0)
            .step_increment(1.0).page_increment(1.0)
            .value(1.0)
            .build();
        let spin_m = SpinButton::builder()
            .adjustment(&mod_adj_m)
            .digits(0).wrap(true).width_request(2).visible(true)
            .build();
        let mod_adj_y = Adjustment::builder()
            .lower(1970.0).upper(9999.0)
            .step_increment(1.0).page_increment(1.0)
            .value(1.0)
            .build();
        let spin_y = SpinButton::builder()
            .adjustment(&mod_adj_y)
            .digits(0).wrap(true).width_request(4).visible(true)
            .value(2025.0)
            .build();
        Self{
            dir_btn  : Button::with_label("dir"),
            dir_lbl  : Label::builder().css_classes(vec!["dir_lbl"]).label("...").build(),
            year_lbl : Label::new(Some("year:")),
            year_btn : spin_y,
            mon_lbl  : Label::new(Some("mon:")),
            mon_btn  : spin_m,
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
