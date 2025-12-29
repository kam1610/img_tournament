mod imtr_button_box;
mod imtr_event_object;
mod imtr_mediator;
mod imtr_preview;
mod month_img_list;
mod tree_util;

use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;

use gtk::Application;
use gtk::ApplicationWindow;
use gtk::CssProvider;
use gtk::gdk::Display;
use gtk::Label;
use gtk::Orientation;
use gtk::DrawingArea;
use gtk::glib::Object;
use gtk::prelude::*;

use crate::imtr_button_box::ImtrButtonBox;
use crate::imtr_mediator::ImtrMediator;
use crate::month_img_list::get_month_img_files;
use crate::imtr_preview::ImtrPreview;
use crate::tree_util::Node;
use crate::imtr_event_object::ImtrEventObject; // debug

// load_css ////////////////////////////////////////////////
pub fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
// main ////////////////////////////////////////////////////
pub fn build_ui(app: &Application) {

    // let paths = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i"]
    //     .into_iter()
    //     .map(|s| PathBuf::from(s))
    //     .collect::<Vec<_>>();
    // let mut root: Option<Rc<RefCell<Node>>> = None;
    // for p in paths{
    //     root = Some(insert(root.take(), p)); }
    // print_tree(&root.clone().unwrap(), 0);

    //// serialize sample
    //let s = root.expect("serializable root").borrow_mut().to_serializable();
    //println!("{}", serde_json::to_string(&s).unwrap());

    // let p = Path::new("/home/kosame/.config/vivaldi/Default/VivaldiThumbnails");
    // let r = get_month_img_files(p, 2025, 7);
    // println!("{:?}", r);

    ////////////////////////////////////////////////////////

    let vbox = gtk::Box::builder().orientation(Orientation::Vertical).build();

    let pwin = ImtrPreview::new();
    vbox.append(&pwin);

    let btn_box = ImtrButtonBox::new();
    vbox.append(&btn_box);

    let mediator = ImtrMediator::new();
    btn_box.set_mediator(Some(mediator.clone().upcast::<Object>()));


    let window = ApplicationWindow::builder()
        .application(app)
        .title( String::from("img_tournament") )
        .default_width(600)
        .default_height(800)
        .child(&vbox)
        .build();

    mediator.set_property("btn_box", btn_box.clone());
    mediator.set_property("win", window.clone());

    window.present();

    // let evt = ImtrEventObject::new();
    // evt.set_path(Path::new("/dev/shm/test.png"), Path::new("/dev/shm/test.png"));
    // pwin.emit_by_name::<()>("set-images", &[&evt]);



}
