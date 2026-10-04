use niri_ipc::socket::Socket;
use niri_ipc::{Action, KeyboardLayouts, LayoutSwitchTarget, Request, Response};
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct LayoutLib {
    pub layouts: RefCell<Vec<String>>,
    pub layout: RefCell<String>,
    pub layout_short: RefCell<String>,
}

impl LayoutLib {
    pub fn new() -> Result<Self, anyhow::Error> {
        Ok(Self {
            layout: RefCell::new("??".to_string()),
            layout_short: RefCell::new("??".to_string()),
            layouts: RefCell::new(Vec::new()),
        })
    }

    /// Загружает из IPC список доступных раскладок
    pub fn get_layouts(&self) -> Result<(), anyhow::Error> {
        let kb = Self::query_layouts()?;
        *self.layouts.borrow_mut() = kb.names;
        Ok(())
    }

    /// Грузит текущую раскладку из IPC в структуру
    pub fn get_layout(&self) -> Result<(), anyhow::Error> {
        let kb = Self::query_layouts()?;
        let current = kb
            .names
            .get(kb.current_idx as usize)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("current_idx out of range"))?;

        *self.layout_short.borrow_mut() = current.chars().take(2).collect();
        *self.layout.borrow_mut() = current;
        *self.layouts.borrow_mut() = kb.names; // заодно обновим список
        Ok(())
    }

    /// Техническая функция: запрос раскладок у niri
    fn query_layouts() -> Result<KeyboardLayouts, anyhow::Error> {
        let mut socket = Socket::connect()?;
        let reply = socket.send(Request::KeyboardLayouts)?;
        match reply.map_err(|e| anyhow::anyhow!("niri error: {e}"))? {
            Response::KeyboardLayouts(kb) => Ok(kb),
            other => Err(anyhow::anyhow!("Unexpected response: {other:?}")),
        }
    }

    /// Устанавливает конкретную раскладку (по имени)
    #[allow(dead_code)]
    pub fn set_layout(&self, layout_mut: &str) -> Result<(), anyhow::Error> {
        let idx = self
            .layouts
            .borrow()
            .iter()
            .position(|l| l == layout_mut)
            .ok_or_else(|| {
                anyhow::anyhow!("Layout not found in: {:?}", self.layouts.borrow())
            })?;

        let mut socket = Socket::connect()?;
        let reply = socket.send(Request::Action(Action::SwitchLayout {
            layout: LayoutSwitchTarget::Index(idx as u8),
        }))?;
        reply.map_err(|e| anyhow::anyhow!("niri error: {e}"))?;

        *self.layout.borrow_mut() = layout_mut.to_string();
        *self.layout_short.borrow_mut() = layout_mut.chars().take(2).collect();
        Ok(())
    }
}
