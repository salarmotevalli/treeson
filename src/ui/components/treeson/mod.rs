use std::marker::PhantomData;

use ratatui::{
    layout::Rect,
    style::{Color, Modifier},
    widgets::Block,
};
use tui_tree_widget::Tree;

use crate::{ui::Drawable, App};

#[derive(Default)]
pub struct TreesonUI;

impl Drawable<TreesonUI> for App<'_> {
    fn draw(&mut self, f: &mut ratatui::prelude::Frame, r: Rect, _t: PhantomData<TreesonUI>) {
        let items = Tree::new(self.items.clone())
            .expect("all item identifiers are unique")
            .block(Block::bordered().title(format!("Tree Widget {:?}", self.state)))
            .highlight_style(
                ratatui::style::Style::new()
                    .fg(Color::Black)
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("-> ");
        f.render_stateful_widget(items, r, &mut self.state);
    }
}
