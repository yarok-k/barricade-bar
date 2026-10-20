use anyhow::Result;

pub trait WidgetTrait: Sized {
    fn new(popup: Option<String>) -> Result<Self>;
    fn update(&mut self) -> Result<()>;
}
