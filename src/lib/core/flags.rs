use clap::Parser;

#[derive(Parser, Debug, Clone, Default)]
#[command(name = "barricade", about = "Barricade statusbar")]
pub struct CliArgs {
    /// Пересобрать интерфейс и обновить список мониторов
    #[arg(short = 'u', long = "update")]
    pub update: bool,
}

pub fn parse_flags() -> CliArgs {
    CliArgs::parse()
}
