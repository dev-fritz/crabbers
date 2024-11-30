use ratatui::{
    buffer::Buffer,
    layout::{Constraint::Percentage, Alignment, Layout, Rect},
    widgets::{Block, Paragraph, Table, Widget},
    style::{Color, Stylize}};

pub fn run(area: Rect, buf: &mut Buffer, block: Block) {
    let horizontal = Layout::horizontal([Percentage(20), Percentage(60), Percentage(20)]);
    let [_, block_area, _] = horizontal.areas(area);

    let vertical = Layout::vertical([Percentage(12), Percentage(48), Percentage(22), Percentage(18)]);
    let [_, logo_area, title_area, buttons_area] = vertical.areas(block_area);

    Table::default()
        .bg(Color::DarkGray)
        .block(block)
        .render(area, buf);

    Paragraph::new(
        "
████████  ████████     ██     ███████   ███████   ████████  ████████   ███████\n\
█         █       █   █  █    █      █  █      █  █         █       █  █      \n\
█         ████████   █    █   ███████   ███████   ███████   ████████   ███████\n\
█         █     █   █ █  █ █  █      █  █      █  █         █     █          █\n\
████████  █      █ █        █ ███████   ███████   ████████  █      █   ███████\n ")
        .alignment(Alignment::Center)
        .render(logo_area, buf);


    Paragraph::new("Bem-vindo ao Crabbers")
        .alignment(Alignment::Center)
        .render(title_area, buf);

    {
        Paragraph::new("Linha dos Botões")
            .alignment(Alignment::Center)
            .render(buttons_area, buf);
    }
}