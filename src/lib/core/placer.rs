use std::collections::HashMap;
use std::path::Path;
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

use super::parcer::{ElementType, Parser};
pub use super::parcer::ElementPos;

pub struct Placer {
    pub widgets: HashMap<ElementPos, Vec<BuiltWidget>>,
}

impl Placer {
    pub fn new(containers: &[(ElementPos, &gtk::Box)]) -> Result<Self> {
        let mut placer = Self {
            widgets: HashMap::new(),
        };

        let config = Parser::new().parse()?;

        // 1. Собираем виджеты из распарсенного конфига
        for (pos, specs) in config.elements {
            let mut built_list = Vec::new();
            for spec in specs {
                built_list.push(Self::widget_assembler(&spec.kind, spec.command)?);
            }
            placer.widgets.insert(pos, built_list);
        }

        // 2. Раскладываем по контейнерам
        for (pos, container) in containers {
            if let Some(widgets) = placer.widgets.get(pos) {
                for widget in widgets {
                    container.add(&widget.gtk_widget());
                }
            }
        }

        // 3. Подключаем стили, если указаны
        if let Some(path) = config.stylesheet {
            Self::apply_stylesheet(&path);
        }

        let _ = placer.update_widgets();
        Ok(placer)
    }

    // Ошибка в стилях не должна ронять бар, поэтому только предупреждаем
    fn apply_stylesheet(path: &Path) {
        if !path.is_file() {
            eprintln!("stylesheet not found: {}", path.display());
            return;
        }

        let Some(screen) = gtk::gdk::Screen::default() else {
            eprintln!("stylesheet skipped: no default screen");
            return;
        };

        let provider = gtk::CssProvider::new();
        match provider.load_from_path(&path.to_string_lossy()) {
            std::result::Result::Ok(()) => gtk::StyleContext::add_provider_for_screen(
                &screen,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_USER,
            ),
            Err(e) => eprintln!("failed to load stylesheet {}: {e}", path.display()),
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
