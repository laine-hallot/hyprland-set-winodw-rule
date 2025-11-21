use crate::wayland;
use crate::wayland::ClientRegion;
use crate::wayland::State as WlState;
use hyprland::data::*;
use hyprland::prelude::*;
use hyprland::shared::Address;
use ratatui::prelude::*;
use std::collections::HashMap;
use wayland_client::EventQueue;

use color_eyre::Result;
use ratatui::{Frame, widgets::Paragraph};

#[derive(Debug, Default)]
enum ViewState {
    #[default]
    WindowSelect,
}

#[derive(Debug, Default)]
struct Model {
    selected_window: String,
    running_state: RunningState,
    view: ViewState,
    wl_state: WlState,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(PartialEq)]
enum Message {
    Select,
    Quit,
    //window_selector::create_window();
}

pub fn tui_root() -> Result<Option<Client>> {
    tui::install_panic_hook();
    color_eyre::install()?;
    let client_result = app();

    tui::restore_terminal()?;
    return client_result;
}

fn index_monitors_by_client_id(monitors: Monitors, clients: Clients) -> HashMap<Address, Monitor> {
    return HashMap::<Address, Monitor>::from_iter(clients.iter().filter_map(|client| {
        match monitors.iter().find(|monitor| {
            client
                .monitor
                .is_some_and(|client_monitor_id| monitor.id == client_monitor_id)
        }) {
            Some(monitor) => Some((client.address.clone(), monitor.clone())),
            None => None,
        }
    }));
}
fn index_client_id(clients: &Clients) -> HashMap<Address, Client> {
    return HashMap::<Address, Client>::from_iter(
        clients
            .iter()
            .map(|client| (client.address.clone(), client.clone())),
    );
}

fn app() -> Result<Option<Client>> {
    let monitors = Monitors::get()?;
    let clients = Clients::get()?;

    let window_select_inputs =
        wayland::window_selector::create_state_and_region_bounds(&clients, &monitors);
    let event_queue = wayland::window_selector::create_wayland_window_select();

    let mapped_client_id_and_client = index_client_id(&clients);
    let terminal = tui::init_terminal()?;

    let render_result = render(
        window_select_inputs,
        event_queue,
        mapped_client_id_and_client,
        terminal,
    )?;

    if let Some(click_position) = render_result {
        println!("Processing...");
        let selected_client = clients.iter().find(|client| {
            let click_x = click_position.0.trunc() as i16;
            let click_y = click_position.1.trunc() as i16;
            let x = client.at.0 < click_x && click_x < (client.at.0 + client.size.0);
            let y = client.at.1 < click_y && click_y < (client.at.1 + client.size.1);
            return x && y;
        });
        return Ok(match selected_client {
            Some(selected_client) => Some(selected_client.clone()),
            None => None,
        });
    }
    return Ok(None);
}

fn view(model: &mut Model, frame: &mut Frame) {
    // println!("Press <ESC> to quit.");
    let span1 = "Select a window: ".bold();
    let span2 = format!("{}", model.selected_window).bold();
    let line = Line::from(vec![span1, span2]);
    let text = Text::from(line);
    frame.render_widget(Paragraph::new(text), frame.area());
}

fn update(
    model: &mut Model,
    msg: String,
    client_regions: Vec<ClientRegion>,
    mapped_client_id_and_client: HashMap<Address, Client>,
) -> Option<Message> {
    model.selected_window = msg;
    if let (Some(pointer_position), Some((pointer_monitor_id, _))) = (
        model.wl_state.pointer_position,
        model.wl_state.pointer_surface.clone(),
    ) {
        //dbg!(&pointer_monitor_id);
        let hovered_client_region = client_regions.iter().find(|client| {
            let pointer_x = pointer_position.0.trunc() as i16;
            let pointer_y = pointer_position.1.trunc() as i16;
            let x = client.at.0 < pointer_x && pointer_x < (client.at.0 + client.size.0);
            let y = client.at.1 < pointer_y && pointer_y < (client.at.1 + client.size.1);
            if let Some(client_monitor) = &client.monitor {
                return x && y && client_monitor.to_string() == pointer_monitor_id;
            }
            return false;
        });
        let hovered_client = match hovered_client_region {
            Some(hovered_client_region) => {
                mapped_client_id_and_client.get(&hovered_client_region.client_id)
            }
            None => None,
        };
        model.selected_window = match hovered_client {
            Some(client) => client.title.clone(),
            None => "".to_string(),
        };
    }
    /* match msg {
        Message::Select => {}
        Message::Quit => {
            // You can handle cleanup and exit here
            model.running_state = RunningState::Done;
        }
    }; */
    None
}

fn render(
    (state, client_regions): (WlState, Vec<ClientRegion>),
    mut event_queue: EventQueue<WlState>,
    mapped_client_id_and_client: HashMap<Address, Client>,
    mut terminal: Terminal<impl Backend>,
) -> Result<Option<(f64, f64)>> {
    let mut model = Model::default();
    model.wl_state = state;
    while model.wl_state.running {
        event_queue
            .blocking_dispatch(&mut model.wl_state)
            .expect("window loop");

        update(
            &mut model,
            "None".to_string(),
            client_regions.clone(),
            mapped_client_id_and_client.clone(),
        );

        terminal.draw(|f| view(&mut model, f))?;
    }

    Ok(model.wl_state.pointer_position.clone())
}

mod tui {
    use ratatui::{
        Terminal,
        backend::{Backend, CrosstermBackend},
        crossterm::{
            ExecutableCommand,
            terminal::{
                EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
            },
        },
    };
    use std::{io::stdout, panic};

    pub fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(terminal)
    }

    pub fn restore_terminal() -> color_eyre::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn install_panic_hook() {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            stdout().execute(LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();
            original_hook(panic_info);
        }));
    }
}
