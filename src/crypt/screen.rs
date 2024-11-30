use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Block, Table, Widget}
};

pub fn run(area: Rect, buf: &mut Buffer, block: Block) {
    Table::default()
        .bg(Color::DarkGray)
        .block(block)
        .render(area, buf);
}