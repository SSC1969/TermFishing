use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect, Spacing},
    symbols::merge::MergeStrategy,
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::{App, Menu};

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        // let block = Block::bordered()
        //     .title("Fishin'")
        //     .title_alignment(Alignment::Center)
        //     .border_type(BorderType::Rounded);
        //
        // let inner = block.inner(area);
        let [main, toolbar] = Layout::vertical([Constraint::Fill(1), Constraint::Max(5)])
            .spacing(Spacing::Overlap(1))
            .areas(area);
        let [viewport, menu] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Percentage(25)])
                .spacing(Spacing::Overlap(1))
                .areas(main);

        self.render_viewport(viewport, buf);
        self.render_menu(menu, buf);
        self.render_toolbar(toolbar, buf);
    }
}

impl App {
    fn render_viewport(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Fishin'")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .merge_borders(MergeStrategy::Exact);

        Paragraph::new("Yuh I'm fishin' it")
            .centered()
            .block(block)
            .render(area, buf);
    }

    fn render_toolbar(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .merge_borders(MergeStrategy::Exact);

        Paragraph::new("Yuh I'm fishin' it")
            .centered()
            .block(block)
            .render(area, buf);
    }

    fn render_menu(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(format!("{:?}", self.menu))
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .merge_borders(MergeStrategy::Exact);

        (match self.menu {
            Menu::Home => Paragraph::new("Home"),
            Menu::Inventory => Paragraph::new("Inventory"),
            Menu::Collection => Paragraph::new("Collection"),
            Menu::Options => Paragraph::new("Options"),
        })
        .block(block)
        .centered()
        .render(area, buf);
    }
}
