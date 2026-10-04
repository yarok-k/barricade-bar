use anyhow::Result;
use gtk::prelude::*;
use std::cell::RefCell;
use crate::lib::layout::LayoutLib;
use crate::lib::core::widget::WidgetTrait;

#[derive(Debug, Clone)]
pub struct Layout {
    layoutlib: LayoutLib,
    pub trigger: gtk::Button,
    pub text: gtk::Label,
    pub layout: RefCell<String>,
    pub layouts: RefCell<Vec<String>>,
}
impl WidgetTrait for Layout {
    fn new() -> Result<Self> {
        let trigger = gtk::Button::new();
        let text = gtk::Label::new(Some("??"));
        trigger.set_child(Some(&text));
        let layoutlib = LayoutLib::new()?;
        let mut layout = Self {
            layoutlib: layoutlib,
            trigger,
            text,
            layout: RefCell::new("??".to_string()),
            layouts: RefCell::new(Vec::new()),
        };
        layout.update()?;
        Ok(layout)
    }
    fn update(&mut self) -> Result<()> {
        self.layoutlib.get_layout()?;
        self.layoutlib.get_layouts()?;
        *self.layout.borrow_mut() = self.layoutlib.layout_short.borrow().clone();
        *self.layouts.borrow_mut() = self.layoutlib.layouts.borrow().clone();
        self.text.set_label(self.layout.borrow().as_str());
        Ok(())
    }
}
