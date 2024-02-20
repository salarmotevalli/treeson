mod preload;
mod ui;

use clap::Parser;
use crossterm::event::KeyCode;
use preload::*;
use ratatui::prelude::{CrosstermBackend, Terminal};
use serde_json::{from_reader, Value};
use std::collections::BTreeMap;
use tui_tree_widget::{TreeItem, TreeState};
use uuid::Uuid;

#[derive(Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None, arg_required_else_help = !is_data_piped())]
pub struct Cli {
    #[arg(short = 'i')]
    pub input: Option<String>,
}
struct App<'a> {
    state: TreeState<String>,
    items: Vec<TreeItem<'a, String>>,
    should_quit: bool,
}

fn startup() -> anyhow::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> anyhow::Result<()> {
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn update(app: &mut App) -> anyhow::Result<()> {
    // Check for user input
    if crossterm::event::poll(std::time::Duration::from_millis(500))? {
        // If a key event occurs, handle it
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.kind == crossterm::event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('\n' | ' ') => app.state.toggle_selected(),
                    KeyCode::Left => app.state.key_left(),
                    KeyCode::Right => app.state.key_right(),
                    KeyCode::Down => app.state.key_down(&app.items),
                    KeyCode::Up => app.state.key_up(&app.items),
                    KeyCode::Home => {
                        app.state.select_first(&app.items);
                    }
                    KeyCode::End => {
                        app.state.select_last(&app.items);
                    }
                    KeyCode::PageDown => app.state.scroll_down(3),
                    KeyCode::PageUp => app.state.scroll_up(3),
                    KeyCode::Char('q') => app.should_quit = true,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    // Parse flags
    let matches = Cli::parse();

    // Store entry data
    let entry_buf: Box<dyn Read> = define_reader(matches.input);
    //
    // validate and decode json
    let data_map: BTreeMap<String, Value> = from_reader(entry_buf).expect("fuck");

    startup()?;
    // Initialize the terminal backend using crossterm
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // Define our counter variable
    // This is the state of our application

    // application state
    let mut app = App {
        state: TreeState::default(),
        items: to_tree_items(data_map),
        should_quit: false,
    };

    // Main application loop
    loop {
        // Render the UI
        terminal.draw(|f| {
            // app.draw(f, f.size());
            ui::draw_components(f, &mut app, f.size())
        })?;

        update(&mut app)?;

        // application exit
        if app.should_quit {
            break;
        }
    }

    shutdown()?;

    Ok(())
}

fn to_tree_items(data_map: BTreeMap<String, Value>) -> Vec<TreeItem<'static, String>> {
    let mut items: Vec<TreeItem<'static, String>> = Vec::new();

    for (key, value) in data_map {
        match value {
            Value::Null => items.push(TreeItem::new_leaf(
                Uuid::new_v4().to_string(),
                key,
                Some("".into()),
            )),
            Value::Number(d) => items.push(TreeItem::new_leaf(
                Uuid::new_v4().to_string(),
                key,
                Some(d.into()),
            )),
            Value::String(d) => items.push(TreeItem::new_leaf(
                Uuid::new_v4().to_string(),
                key,
                Some(d.into()),
            )),
            Value::Bool(d) => items.push(TreeItem::new_leaf(
                Uuid::new_v4().to_string(),
                key,
                Some(d.into()),
            )),
            Value::Array(d) => array_to_tree(&mut items, d, Some(key)),
            Value::Object(d) => map_to_tree(&mut items, d, Some(key)),
        };
    }

    items
}

fn array_to_tree(items: &mut Vec<TreeItem<'static, String>>, array: Vec<Value>, key: Option<String>) {
    let mut array_items: Vec<TreeItem<'static, String>> = Vec::new();
    array.into_iter().for_each(|value| {
        match value {
            Value::Null => array_items.push(TreeItem::new_leaf(Uuid::new_v4().to_string(), "", None)),
            Value::Number(d) => {
                array_items.push(TreeItem::new_leaf(Uuid::new_v4().to_string(), format!("{d}"), None))
            }
            Value::String(d) => array_items.push(TreeItem::new_leaf(Uuid::new_v4().to_string(), d, None)),
            Value::Bool(d) => array_items.push(TreeItem::new_leaf(
                Uuid::new_v4().to_string(),
                if d == true {
                    concat!(true)
                } else {
                    concat!(false)
                },
                None,
            )),
            Value::Array(d) => array_to_tree(&mut array_items, d, None),
            Value::Object(d) => map_to_tree(&mut array_items, d, None),
        };

        items.push(
            TreeItem::new(Uuid::new_v4().to_string(), if let Some(k) = key.clone() {k}else{"|".to_string()}, array_items.clone()).expect("array-to-tree"),
        );
    });
}

fn map_to_tree(items: &mut Vec<TreeItem<'static, String>>, map: serde_json::Map<String, Value>, map_key: Option<String>) {
    let mut map_items: Vec<TreeItem<'static, String>> = Vec::new();
    map.into_iter().for_each(|(key, value)| {
        match value {
            Value::Null => map_items.push(TreeItem::new_leaf(Uuid::new_v4().to_string(), key, Some("".into()))),
            Value::Number(d) => {
                map_items.push(TreeItem::new_leaf(Uuid::new_v4().to_string(), key, Some(d.into())))
            }
            Value::String(d) => {
                map_items.push(TreeItem::new_leaf(Uuid::new_v4().to_string(), key, Some(d.into())))
            }
            Value::Bool(d) => {
                map_items.push(TreeItem::new_leaf(Uuid::new_v4().to_string(), key, Some(d.into())))
            }
            Value::Array(d) => array_to_tree(&mut map_items, d, Some(key)),
            Value::Object(d) => map_to_tree(&mut map_items, d, Some(key)),
        };
    });

    items.push(TreeItem::new(Uuid::new_v4().to_string(), if let Some(k) = map_key {k}else{"|".to_string()}, map_items.clone()).expect("mao_to_tree"));
}
