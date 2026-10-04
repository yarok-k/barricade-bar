use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::{Result,Ok};

use gtk::prelude::*;
use serde::Deserialize;

use crate::lib::core::element::BuiltWidget;
use crate::lib::core::widget::WidgetTrait;

use crate::widgets::{
    clock::Clock,
    audio::Audio
};

// Позиции элементов
#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ElementPos {
    Left,
    Center,
    Right,
}

// Типы элементов для парсинга YAML
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ElementType {
    Clock,
    Audio,
}

// Структуры описания YAML-файла
#[derive(Debug, Deserialize)]
pub struct Config {
    pub elements: Elements,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub struct Elements {
    #[serde(flatten)]
    pub positions: HashMap<ElementPos, Option<Vec<ElementType>>>,
}

pub struct Placer {
    pub widgets: HashMap<ElementPos, Vec<BuiltWidget>>,
}

impl Placer {
    pub fn new(containers: &[(ElementPos, &gtk::Box)]) -> Result<Self> {
        let mut placer = Self {
            widgets: HashMap::new(),
        };

        let raw_content = placer.reader()?;
        let content: Config = serde_yaml::from_str(&raw_content)?;

        // 1. Собираем виджеты по типам из YAML
        for (pos, element_types) in content.elements.positions {
            if let Some(types) = element_types {
                let mut built_list = Vec::new();
                for elem_type in types {
                    let built_widget = Self::widget_assembler(&elem_type);
                    built_list.push(built_widget);
                }
                placer.widgets.insert(pos, built_list);
            }
        }
        // 2. Вставляем их в переданные GTK-контейнеры
        for (pos, container) in containers {
            if let Some(widgets) = placer.widgets.get(pos) {
                for widget in widgets {
                    container.add(&widget.gtk_widget());
                    if let BuiltWidget::Clock(clock_obj) = widget {
                        clock_obj.update();
                    }
                }
            }
        }

        Ok(placer)
    }
    pub fn update_widgets(&mut self) -> Result<()> {
        for (_, widgets) in self.widgets.iter_mut() {
            for widget in widgets {
                match widget {
                    BuiltWidget::Clock(widg) => {
                        widg.update();
                        return Ok(());
                    },
                    BuiltWidget::Audio(widg) => {
                        widg.update();
                        return Ok(());
                    }
                    _ => {
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    }

    fn reader(&self) -> Result<String, std::io::Error> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let config_dir = Path::new(&home).join(".config/barricade");
        let file_path = config_dir.join("bar.bst");

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        if !file_path.exists() {
            let default_content = "elements:\n  left:\n  center:\n    - \"clock\"\n  right:\n";
            fs::write(&file_path, default_content)?;
        }

        fs::read_to_string(&file_path)
    }

    pub fn widget_assembler(element_type: &ElementType) -> BuiltWidget {
        match element_type {
            ElementType::Clock => {
                let widget_ = Clock::new();
                BuiltWidget::Clock(widget_.expect("REASON"))
            }
            ElementType::Audio => {
                let widget_  = Audio::new();
                BuiltWidget::Audio(widget_.expect("REASON"))
            }
        }
    }
}
