use anyhow::Result;
use gtk::prelude::*;


pub trait WidgetTrait: Sized {
    fn new(popup: Option<String>) -> Result<Self>;
    fn update(&mut self) -> Result<()>;
    fn make_trigger() -> (gtk::Box, gtk::Button) {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let trigger = gtk::Button::new();
        trigger.style_context().add_class("barricade_statusbar_button");
        trigger.add(&container);
        (container, trigger)
    }
}
