use sound::app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new()?.run(&mut { terminal });
    ratatui::restore();
    result
}
