use std::collections::HashMap;
use std::time::SystemTime;
use anyhow::{Result, Ok};

use gtk::prelude::*;

use crate::lib::core::element::BuiltWidget;
use crate::lib::core::widget::WidgetTrait;

use crate::widgets::battery::Battery;
use crate::widgets::network::Network;
use crate::widgets::{
    clock::Clock,
    audio::Audio,
    layout::Layout,
};

use super::parcer::{modified_time, ElementType, Parser};
pub use super::parcer::ElementPos;

pub struct Placer {
    pub widgets: HashMap<ElementPos, Vec<BuiltWidget>>,
    // gtk-объекты — это счётные ссылки, поэтому клонировать контейнеры дёшево
    containers: Vec<(ElementPos, gtk::Box)>,
    parser: Parser,
    // Время изменения конфига на момент последней сборки виджетов
    config_mtime: Option<SystemTime>,
}

impl Placer {
    pub fn new(containers: &[(ElementPos, &gtk::Box)]) -> Result<Self> {
        let mut placer = Self {
            widgets: HashMap::new(),
            containers: containers
                .iter()
                .map(|(pos, c)| (*pos, (**c).clone()))
                .collect(),
            parser: Parser::new(),
            config_mtime: None,
        };

        placer.rebuild()?;
        Ok(placer)
    }

    // Читает конфиг, собирает виджеты и раскладывает их по контейнерам.
    // Старые виджеты заменяются только после успешной сборки новых,
    // поэтому при ошибке в конфиге бар остаётся прежним.
    fn rebuild(&mut self) -> Result<()> {
        // mtime читаем до парсинга, чтобы неудачная попытка не повторялась каждый тик
        self.config_mtime = modified_time(self.parser.config_path());
        let config = self.parser.parse()?;

        // 1. Собираем виджеты из распарсенного конфига
        let mut widgets = HashMap::new();
        for (pos, specs) in config.elements {
            let mut built_list = Vec::new();
            for spec in specs {
                built_list.push(Self::widget_assembler(&spec.kind, spec.command)?);
            }
            widgets.insert(pos, built_list);
        }

        // 2. Очищаем контейнеры от старых виджетов
        for (_, container) in &self.containers {
            for child in container.children() {
                container.remove(&child);
            }
        }

        // 3. Раскладываем новые по контейнерам
        for (pos, container) in &self.containers {
            if let Some(list) = widgets.get(pos) {
                for widget in list {
                    container.add(&widget.gtk_widget());
                }
            }
            // окно уже могло быть показано, новые виджеты сами не появятся
            container.show_all();
        }

        self.widgets = widgets;
        let _ = self.update_widgets();
        Ok(())
    }

    // Пересобирает бар, если файл конфига изменился
    pub fn reload_if_changed(&mut self) {
        if modified_time(self.parser.config_path()) == self.config_mtime {
            return;
        }
        if let Err(e) = self.rebuild() {
            eprintln!("config reload failed: {e:#}");
        }
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
