use std::marker::PhantomData;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
};

use crate::{ui::Drawable, App};

#[derive(Default)]
pub struct Header;

impl Drawable<Header> for App<'_>{
    fn draw(&mut self, f: &mut ratatui::prelude::Frame, r: Rect, _t: PhantomData::<Header>){
        let top_inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ])
            .split(r);

        f.render_widget(
            Paragraph::new("Quite: q".to_string()).block(Block::new().borders(Borders::all())),
            top_inner_layout[0],
        );
    }
    // add code here
}
