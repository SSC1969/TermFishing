use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect, Spacing},
    style::{Color, Style},
    symbols::merge::MergeStrategy,
    text::Line,
    widgets::{Block, BorderType, List, ListItem, Padding, Paragraph, StatefulWidget, Widget},
};

use crate::app::{App, InputMode, MENU_SIZE, Menu};

impl Widget for &mut App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [main, toolbar] = Layout::vertical([Constraint::Fill(1), Constraint::Max(3)])
            .spacing(Spacing::Overlap(1))
            .areas(area);
        let [viewport, menu] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Percentage(25)])
                .spacing(Spacing::Overlap(1))
                .areas(main);

        let [_, viewport_bottom] =
            Layout::vertical([Constraint::Fill(1), Constraint::Max(8)]).areas(viewport);

        let [_, chatbox] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Max(40)]).areas(viewport_bottom);

        let [messages, input] =
            Layout::vertical([Constraint::Fill(1), Constraint::Max(3)]).areas(chatbox);

        self.render_viewport(viewport, buf);
        self.render_menu(menu, buf);
        self.render_toolbar(toolbar, buf);
        self.render_messages(messages, buf);
        self.render_input(input, buf);
    }
}

impl App {
    // move to bottom of impl
    fn render_input(&mut self, area: Rect, buf: &mut Buffer) {
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let style = match self.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Color::Yellow.into(),
        };
        let input = Paragraph::new(self.input.value())
            .style(style)
            .scroll((0, scroll as u16))
            .block(Block::bordered());
        input.render(area, buf);

        if self.input_mode == InputMode::Editing {
            let x = self.input.visual_cursor().max(scroll) - scroll + 1;
            self.cursor_position = Some((area.x + x as u16, area.y + 1));
        }
    }

    fn render_messages(&self, area: Rect, buf: &mut Buffer) {
        let start = self.messages.len().saturating_sub(3);
        let messages = self.messages[start..].iter().map(String::as_str);
        let messages = List::new(messages).block(Block::bordered().title("Messages"));
        // multiple renders()'s so specify widget
        Widget::render(messages, area, buf);
    }

    fn render_chat(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new();
        let chat = "
Adam caught [LEGENDARY] Salmon
Adam: yooooo
Sam: dub";

        Paragraph::new(chat).block(block).render(area, buf);
    }

    fn render_viewport(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Fishin'")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .merge_borders(MergeStrategy::Exact);

        let _cat = r#"
　　 　　  　　, -ｰ,
　　　　　　 ／　　 |
　　 ∧∧　　／　　　 |
　　(*ﾟ－ﾟ)／.　 　 |
　　 |　つ'@　　 　 |
　 ～＿`)｀).　  　 |
￣￣￣しU　　　　 　 |
　　　　　|　 　　　 |
～～～～～～～～～～～～
"#;

        let cat2 = r#"

　　　　　　　　　　　　　　　　　　　 ,
　　　　　　　　　　　　　　 　 　　,／ヽ
　　　　　　　　　　　　　 　 　 ,／　　　ヽ
　　　　 　　　　　  ∧＿∧　　,／　　　　　　ヽ
　　　　　 　　　　（ ´∀｀）,／　 　　　　　　 ヽ
　　　　　　　 　　（　　つつ@　 　　　　 　　　　ヽ
　　　 　 　＿＿   ｜ ｜ |　　　 　　　　 　　　　  ヽ
　　　　　　|――|　（_＿）＿）　　　　 　　 　　　　　　ヽ
￣￣￣￣￣￣￣￣￣￣￣￣|　　　　　　　　 　　　 　   　o
／⌒＼／⌒＼／⌒＼／⌒＼|彡~ﾟ　゜~ ~。゜　~ ~　~ ~~　~ ~ ~ ~　~ ~　~ ~~　~゜~ ~。゜　~ ~　~ ~~　~゜~ ~。゜
"#;
        Paragraph::new(cat2)
            // .centered()
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
        Line::from("<b> Backpack").centered().render(layout[1], buf);
        Line::from("<d> Dex").centered().render(layout[2], buf);
        Line::from("<o> Options").centered().render(layout[3], buf);

        block.render(area, buf);
    }

    fn render_menu(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(format!("{:?}", self.menu))
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .merge_borders(MergeStrategy::Exact)
            .padding(Padding::horizontal(1));

        match self.menu {
            Menu::Home => Paragraph::new("Home")
                .centered()
                .block(block)
                .render(area, buf),
            Menu::Backpack => {
                let list_items = self
                    .player
                    .backpack
                    .items
                    .values()
                    .map(|set| set.iter())
                    .flatten()
                    .map(|item| ListItem::from(item));
                let list = List::new(list_items)
                    .highlight_style(Style::new().reversed())
                    .block(block);
                StatefulWidget::render(list, area, buf, &mut self.backpack_state);
            }
            Menu::Dex => {
                let list_items = self
                    .player
                    .dex
                    .get_all()
                    .into_iter()
                    .map(|entry| ListItem::from(entry));
                let list = List::new(list_items)
                    .highlight_style(Style::new().reversed())
                    .block(block);
                StatefulWidget::render(list, area, buf, &mut self.dex_state);
            }
            Menu::Options => Paragraph::new("Options")
                .centered()
                .block(block)
                .render(area, buf),
        }
    }
}
