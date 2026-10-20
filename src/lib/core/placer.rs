use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::{Result, Ok};

use gtk::prelude::*;
use serde::Deserialize;

use crate::lib::core::element::BuiltWidget;
use crate::lib::core::widget::WidgetTrait;

use crate::widgets::battery::Battery;
use crate::widgets::network::Network;
use crate::widgets::{
    clock::Clock,
    audio::Audio,
    layout::Layout,
};

// Позиции элементов
#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ElementPos {
    Left,
    Center,
    Right,
}

// Типы элементов для парсинга YAML.
// Hash нужен, потому что тип используется как ключ HashMap в ElementEntry.
#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ElementType {
    Clock,
    Audio,
    Layout,
    Network,
    Battery,
}

// Элемент списка в YAML. Поддерживаются оба формата:
//   - "clock"                      (без команды)
//   - audio: "/path/to/command"    (с командой, которая выполнится по клику)
//   - layout: None                 (команды нет)
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ElementEntry {
    Plain(ElementType),
    WithCommand(HashMap<ElementType, Option<String>>),
}

impl ElementEntry {
    // Приводим оба варианта к списку пар (тип, команда)
    fn into_pairs(self) -> Vec<(ElementType, Option<String>)> {
        match self {
            ElementEntry::Plain(t) => vec![(t, None)],
            ElementEntry::WithCommand(map) => map
                .into_iter()
                .map(|(t, cmd)| (t, Self::normalize(cmd)))
                .collect(),
        }
    }

    // "None", "" и null означают "команды нет"
    fn normalize(cmd: Option<String>) -> Option<String> {
        cmd.map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty() && !s.eq_ignore_ascii_case("none"))
    }
}

// Структуры описания YAML-файла
#[derive(Debug, Deserialize)]
pub struct Config {
    pub elements: Elements,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Elements {
    #[serde(flatten)]
    pub positions: HashMap<ElementPos, Option<Vec<ElementEntry>>>,
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

        // 1. Собираем виджеты из YAML
        for (pos, entries) in content.elements.positions {
            if let Some(entries) = entries {
                let mut built_list = Vec::new();
                for entry in entries {
                    for (elem_type, popup) in entry.into_pairs() {
                        built_list.push(Self::widget_assembler(&elem_type, popup)?);
                    }
                }
                placer.widgets.insert(pos, built_list);
            }
        }

        for (pos, container) in containers {
            if let Some(widgets) = placer.widgets.get(pos) {
                for widget in widgets {
                    container.add(&widget.gtk_widget());
                }
            }
        }

        let _ = placer.update_widgets();
        Ok(placer)
    }

    pub fn update_widgets(&mut self) -> Result<()> {
        for widgets in self.widgets.values_mut() {
            for widget in widgets.iter_mut() {
                let res = match widget {
                    BuiltWidget::Clock(w) => w.update(),
                    BuiltWidget::Audio(w) => w.update(),
                    BuiltWidget::Layout(w) => w.update(),
                    BuiltWidget::Network(w) => w.update(),
                    BuiltWidget::Battery(w) => w.update(),
                };
                if let Err(e) = res {
                    eprintln!("widget update failed: {e:#}");
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
    pub fn widget_assembler(
        element_type: &ElementType,
        popup: Option<String>,
    ) -> Result<BuiltWidget> {
        Ok(match element_type {
            ElementType::Clock   => BuiltWidget::Clock(Clock::new(popup)?),
            ElementType::Audio   => BuiltWidget::Audio(Audio::new(popup)?),
            ElementType::Layout  => BuiltWidget::Layout(Layout::new(popup)?),
            ElementType::Network => BuiltWidget::Network(Network::new(popup)?),
            ElementType::Battery => BuiltWidget::Battery(Battery::new(popup)?),
        })
    }
}
