use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Stylize},
    symbols,
    text::Line,
    widgets::{Block, Padding, Tabs, Widget},
    DefaultTerminal,
};
use ratatui::widgets::Table;
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

pub fn run() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

#[derive(Default)]
struct App {
    state: AppState,
    selected_tab: SelectedTab,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
enum SelectedTab {
    #[default]
    #[strum(to_string = "0. Start")]
    Start,
    #[strum(to_string = "1. Services")]
    Services,
    #[strum(to_string = "2. Network")]
    Network,
    #[strum(to_string = "3. PDF")]
    PDF,
    #[strum(to_string = "4. Crypt")]
    Crypt,
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
                    KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
                    KeyCode::Char('0') => self.start(),
                    KeyCode::Char('1') => self.services(),
                    KeyCode::Char('2') => self.network(),
                    KeyCode::Char('3') => self.pdf(),
                    KeyCode::Char('4') => self.crypt(),
                    KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    pub fn start(&mut self) {
        self.selected_tab = SelectedTab::Start;
    }

    pub fn network(&mut self) {
        self.selected_tab = SelectedTab::Network;
    }

    pub fn services(&mut self) {
        self.selected_tab = SelectedTab::Services;
    }

    pub fn pdf(&mut self) {
        self.selected_tab = SelectedTab::PDF;
    }

    pub fn crypt(&mut self) {
        self.selected_tab = SelectedTab::Crypt;
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl SelectedTab {
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        self.selected_tab.render(inner_area, buf);
        render_footer(footer_area, buf);
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), self.selected_tab.palette().c700);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    "Created by DevFritz".bold().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("◄ ► to change tab | Press q to quit")
        .centered()
        .render(area, buf);
}

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Self::Start => self.draw_start(area, buf),
            Self::Services => self.draw_service(area, buf),
            Self::Network => self.draw_network(area, buf),
            Self::PDF => self.draw_pdf(area, buf),
            Self::Crypt => self.draw_crypt(area, buf),
        }
    }
}

impl SelectedTab {
    fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    fn draw_start(self, area: Rect, buf: &mut Buffer) {
        Table::default()
            .block(self.block())
            .render(area, buf);
    }

    fn draw_service(self, area: Rect, buf: &mut Buffer) {
        Table::default()
            .block(self.block())
            .render(area, buf);
    }

    fn draw_network(self, area: Rect, buf: &mut Buffer) {
        Table::default()
            .block(self.block())
            .render(area, buf);
    }

    fn draw_pdf(self, area: Rect, buf: &mut Buffer) {
        Table::default()
            .block(self.block())
            .render(area, buf);
    }

    fn draw_crypt(self, area: Rect, buf: &mut Buffer) {
        Table::default()
            .block(self.block())
            .render(area, buf);
    }

    fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }

    const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Start => tailwind::FUCHSIA,
            Self::Services => tailwind::BLUE,
            Self::Network => tailwind::EMERALD,
            Self::PDF => tailwind::INDIGO,
            Self::Crypt => tailwind::RED,
        }
    }
}