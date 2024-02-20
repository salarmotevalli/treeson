mod components;

use ratatui::{layout::Rect, Frame};

use crate::{App, ui::components::{Header, TreesonUI}};

pub trait Drawable<T> {
    fn draw(&mut self, f: &mut Frame, r: Rect, t: std::marker::PhantomData<T>);
}

pub fn draw_components(f: &mut Frame, app: &mut App, r: Rect) {
    use ratatui::prelude::*;

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Min(3), Constraint::Percentage(100)])
        .split(r);

    app.draw(f, layout[0], std::marker::PhantomData::<Header>);
    app.draw(f, layout[1], std::marker::PhantomData::<TreesonUI>);
}

// impl Drawable for App {
    // fn draw(&self, f: &mut Frame, r: Rect) {

        // components::<components::Header>::draw();
        // components::<components::TreesonUI>::draw();
        // self.header.draw(f, layout[0]);
        // self.treeson.draw(f, layout[1]);
    // }
// }
