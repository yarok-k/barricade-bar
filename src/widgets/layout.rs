use crate::lib::layout::LayoutLib;
use gtk::prelude::*;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Layout {
    layoutlib: LayoutLib,
    pub trigger: gtk::Button,
    pub text: gtk::Label,
    pub layout: RefCell<String>,
    pub layouts: RefCell<Vec<String>>,
}
impl Layout {
    pub fn new() -> Result<Self, anyhow::Error> {
        let trigger = gtk::Button::new();
        let text = gtk::Label::new(Some("??"));
        trigger.set_child(Some(&text));
        let layoutlib = LayoutLib::new()?;
        let layout = Self {
            layoutlib: layoutlib,
            trigger,
            text,
            layout: RefCell::new("??".to_string()),
            layouts: RefCell::new(Vec::new()),
        };
        layout.update()?;
        Ok(layout)
    }
    pub fn update(&self) -> Result<(), anyhow::Error> {
        self.layoutlib.get_layout()?;
        self.layoutlib.get_layouts()?;
        *self.layout.borrow_mut() = self.layoutlib.layout_short.borrow().clone();
        *self.layouts.borrow_mut() = self.layoutlib.layouts.borrow().clone();
        self.text.set_label(self.layout.borrow().as_str());
        Ok(())
    }
}
