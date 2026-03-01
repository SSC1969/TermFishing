use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect, Spacing},
    symbols::merge::MergeStrategy,
    text::Line,
    widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

use crate::{
    app::{App, MENU_SIZE, Menu},
    items::Item,
};

struct ItemList {
    items: Vec<Item>,
    state: ListState,
}

impl Widget for &mut App {
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
        let [main, toolbar] = Layout::vertical([Constraint::Fill(1), Constraint::Max(3)])
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

        let inner = block.inner(area).centered_horizontally(Constraint::Fill(1));
        let constraints = (0..MENU_SIZE).map(|_| Constraint::Ratio(1, MENU_SIZE as u32));
        let layout = Layout::horizontal(constraints)
            .flex(Flex::Center)
            .split(inner);

        Line::from("<h> Home").centered().render(layout[0], buf);
        Line::from("<i> Inventory")
            .centered()
            .render(layout[1], buf);
        Line::from("<c> Collection")
            .centered()
            .render(layout[2], buf);
        Line::from("<o> Options").centered().render(layout[3], buf);

        block.render(area, buf);
    }

    fn render_menu(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(format!("{:?}", self.menu))
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .merge_borders(MergeStrategy::Exact);

        match self.menu {
            Menu::Home => Paragraph::new("Home")
                .centered()
                .block(block)
                .render(area, buf),
            Menu::Inventory => {
                let list_items = self
                    .player
                    .backpack
                    .items
                    .values()
                    .map(|set| set.iter())
                    .flatten()
                    .map(|item| ListItem::from(item));
                let list = List::new(list_items).block(block);
                StatefulWidget::render(list, area, buf, &mut self.backpack_state);
            }
            Menu::Collection => Paragraph::new("Collection")
                .centered()
                .block(block)
                .render(area, buf),
            Menu::Options => Paragraph::new("Options")
                .centered()
                .block(block)
                .render(area, buf),
        }
    }
}
