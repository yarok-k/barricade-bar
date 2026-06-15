use hyprland::data::{Devices, Keyboard};
use hyprland::dispatch::*;
use hyprland::shared::HyprData;
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
    /// загружает из IPC пайпа список доступных раскладок
    pub fn get_layouts(&self) -> Result<(), anyhow::Error> {
        let keyboard = self.get_ipc_pipe().ok();
        if let Some(keyboard) = keyboard {
            *self.layouts.borrow_mut() = keyboard
                .layout
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }
        Ok(())
    }

    /// Грузит текущую раскладку из IPC пайпа в структуру
    pub fn get_layout(&self) -> Result<(), anyhow::Error> {
        let keyboard = self.get_ipc_pipe().ok();
        if let Some(keyboard) = keyboard {
            *self.layout.borrow_mut() = keyboard.active_keymap.clone();
            *self.layout_short.borrow_mut() = keyboard.active_keymap.chars().take(2).collect();
        }
        Ok(())
    }
    ///техническая функция для получения данных из IPC пайпа
    fn get_ipc_pipe(&self) -> Result<Keyboard, anyhow::Error> {
        let devices = Devices::get()?;
        let keyboard = devices
            .keyboards
            .into_iter()
            .find(|k| k.main)
            .ok_or_else(|| anyhow::anyhow!("Main keyboard not found"))?;
        Ok(keyboard)
    }

    /// Устанавливает текущую раскладку на конкретную
    #[allow(dead_code)]
    pub fn set_layout(&self, layout_mut: &str) -> Result<(), anyhow::Error> {
        // 1. Проверка
        if !self.layouts.borrow().contains(&layout_mut.to_string()) {
            return Err(anyhow::anyhow!(
                "Layout not found in: {:?}",
                self.layouts.borrow()
            ));
        }

        // 2. Получаем актуальную клавиатуру
        let keyboard = self.get_ipc_pipe()?;

        // 3. Используем Custom диспетчер (самый надежный путь для беты)
        // Это эквивалент команды: hyprctl switchxkblayout <name> <layout>
        Dispatch::call(DispatchType::Custom(
            "switchxkblayout",
            &format!("{} {}", keyboard.name, layout_mut),
        ))?;
        // 4. Обновляем локальный стейт
        *self.layout.borrow_mut() = layout_mut.to_string();

        Ok(())
    }
}
