use crate::backend::state::State;
use tui::backend::Backend;
use tui::layout::Constraint;
use tui::layout::Direction;
use tui::layout::Layout;
use tui::layout::Rect;
use tui::widgets::{Block, Cell, Paragraph, Row, Table};
use tui::Frame;
use tui_input::Input;

use crate::information::Information;

pub fn draw<B: Backend>(f: &mut Frame<B>, state: &State) {
    let screen_size = f.size();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(screen_size.height.max(3) - 3),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(screen_size);

    let topbar = Paragraph::new("TOP BAR");
    let statusbar = Paragraph::new("STATUS BAR");

    let command = Paragraph::new("");

    f.render_widget(topbar, layout[0]);
    draw_information_list(f, state, layout[1], screen_size);
    f.render_widget(statusbar, layout[2]);
    f.render_widget(command, layout[3]);
}

fn draw_information_list<B: Backend>(
    f: &mut Frame<B>,
    state: &State,
    rect: Rect,
    _screen: Rect,
) {
    let count = state.information_list.len();
    let countlen = count.to_string().chars().count();
    let mut rows: Vec<Row> = state
        .information_list
        .iter()
        .enumerate()
        .rev()
        .take((rect.height.max(1) - 1).into())
        .rev()
        .map(|(i, info)| {
            let idx = format!("[{idx:width$}]", idx = i, width = countlen);
            Row::new([Cell::from(idx), Cell::from(info.title())])
        })
        .collect();

    let val = state.input.as_ref().map(Input::value).unwrap_or_default();
    let len = rows.len();
    let input = Row::new([Cell::from(format!("[{}]", count)), Cell::from(val)]);

    rows.push(input);

    let constraints = [
        Constraint::Length(countlen as u16 + 2),
        Constraint::Length(rect.width.max(countlen as u16) - countlen as u16),
    ];

    let table = Table::new(rows)
        .block(Block::default())
        .widths(&constraints);

    f.render_widget(table, rect);
    f.set_cursor(
        rect.x
            + state
                .input
                .as_ref()
                .map(Input::cursor)
                .map(|c| c as u16)
                .unwrap_or(0)
            + countlen as u16
            + 3,
        rect.y + len as u16,
    )
}
