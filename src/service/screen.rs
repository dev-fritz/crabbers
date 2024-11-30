use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Table, Widget},
};
use ratatui::widgets::{Cell, Row};
use crate::service::commands::{list_services, read_services_from_json, Service};

#[derive(Default)]
pub struct TableState {
    headers: Vec<&'static str>,
    rows: Vec<Vec<String>>,
    selected: Option<usize>,
}

impl TableState {
    pub fn new(headers: Vec<&'static str>) -> Self {
        Self {
            headers,
            rows: Vec::new(),
            selected: None,
        }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        if row.len() == self.headers.len() {
            self.rows.push(row);
        } else {
            eprintln!("Erro: Número de colunas na linha não corresponde aos cabeçalhos!");
        }
    }

    pub fn next(&mut self) {
        let i = match self.selected {
            Some(i) => {
                if i >= self.rows.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected = Some(i);
    }

    pub fn previous(&mut self) {
        let i = match self.selected {
            Some(i) => {
                if i == 0 {
                    self.rows.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected = Some(i);
    }

    pub fn render<'a>(&'a self) -> Table<'a> {
        let headers = Row::new(self.headers.iter().map(|&header| Cell::from(header)));
        let rows = self.rows.iter().map(|row| {
            Row::new(row.iter().map(|cell| Cell::from(cell.as_str())).collect::<Vec<_>>())
        });

        let mut table = Table::default()
            .header(headers)
            .rows(rows)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Color::Blue)
                    .title("Services Table")
                    .title_alignment(ratatui::layout::Alignment::Center)
            )
            .widths(
                &self.headers.iter().map(|_| Constraint::Min(10)).collect::<Vec<_>>(),
            );

        if let Some(selected) = self.selected {
            table = table.highlight_symbol(">> ").highlight_style(Color::Yellow);
        }

        table
    }
}

pub fn run(area: Rect, buf: &mut Buffer, block: Block) {
    let services = list_services();
    println!("{:?}", services);

    let mut table_state = TableState::new(vec!["ID", "Name", "Start", "Stop", "Restart"]);

    for service in services {
        table_state.add_row(vec![
            service.id.to_string(),
            service.name.clone(),
            service.start_command.clone().unwrap_or_else(|| "-".to_string()),
            service.stop_command.clone().unwrap_or_else(|| "-".to_string()),
            service.restart_command.clone().unwrap_or_else(|| "-".to_string()),
        ]);
    }

    let chunks = Layout::vertical([
        Constraint::Min(0),       // Services table
    ])
        .split(area);

    render_table(chunks[0], buf, &table_state);
}

fn render_table(area: Rect, buf: &mut Buffer, table_state: &TableState) {
    table_state.render().render(area, buf);
}

pub fn render_popup(area: Rect, buf: &mut Buffer, message: &str, is_error: bool) {
    let popup_block = Block::default()
        .title(if is_error { " Error " } else { " Success " })
        .borders(Borders::ALL)
        .border_style(if is_error {
            Color::Red
        } else {
            Color::Green
        });

    Paragraph::new(Line::raw(message))
        .block(popup_block)
        .alignment(ratatui::layout::Alignment::Center)
        .render(area, buf);
}