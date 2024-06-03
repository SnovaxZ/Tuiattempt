use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    backend::Backend,
    widgets::{Widget, Block, Borders, List, ListItem, Paragraph},
    layout::{Layout, Constraint, Direction},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    Terminal,
    Frame
};
use crossterm::{
    event::{
        self, 
        DisableMouseCapture, 
        EnableMouseCapture,
        Event,
        KeyCode
    },
    execute,
    terminal::{
        disable_raw_mode, 
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    },
};
use unicode_width::UnicodeWidthStr;

enum InputMode {
    Normal,
    Editing,
}

struct App {
    ///current value of inputbox
    input: String,
    ///Current InputMode
    input_mode: InputMode,
    ///History of recorded messages
    messages: Vec<String>,
    //Parameters of function
    parameters: [f32; 4],
    ///State of a function
    state: usize,
    ///bool for switch
    is_on: bool,
}

impl Default for App{
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            parameters: [0.0; 4],
            state: 0,
            is_on: false,
        }
    }
}

fn calculate_dose(weight: f32, max_dose: f32) -> f32 {
    return weight * max_dose
}

fn calculate_max_items(max_dose: f32, item_dose: f32) -> f32 {
    return max_dose / item_dose
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
    [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1)
            ]
            .as_ref(),
        )
        .split(f.size());
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
        vec![
            Span::raw("Press: "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit, "),
            Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to start editing. "),
        ],
        Style::default().add_modifier(Modifier::RAPID_BLINK),
    ),
        InputMode::Editing => (
        vec! [
            Span::raw("Press "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to stop editing. "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to record the message"),
        ],
            Style::default(),
    ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("input"));
    f.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal => {}

        InputMode::Editing => {
            f.set_cursor(
                //put cursor past the end of inputtext
                chunks[1].x + app.input.width() as u16 + 1,
                chunks[1].y + 1,
            )
        }
    }
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    f.render_widget(messages, chunks[2]);

}

fn main() -> Result<(), io::Error> {
    //setup the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::default();
    let res = run_app(&mut terminal, app);

    //terminal.draw(|f| {
    //    ui(f);
    //})?;

    //thread::sleep(Duration::from_millis(5000));

    //restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw (|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        let mut output = format!("");
                        if app.input == "ADI-calc" {
                            app.input.push_str(&format!(" function started, input weight:"));
                            app.is_on = true;
                        } else if app.is_on {
                            output = handleinput(&mut app);
                            app.state += 1;
                            if app.state == 3 {
                                app.state = 0;
                            };
                        };
                        if app.input == "clear" {
                            app.messages.clear()
                        };
                        if app.input == "neofetch" {
                            app.messages.push(format!("                   -`                    snovaxz@archtop"));
                            app.messages.push(format!("                  .o+`                   ---------------"));
                            app.messages.push(format!("                 `ooo/                   OS: Arch Linux x86_64"));
                            app.messages.push(format!("                `+oooo:                  Host: ThinkPad"));
                            app.messages.push(format!("               `+oooooo:                 Kernel: 8.9.3-arch1-1"));
                            app.messages.push(format!("               -+oooooo+:                Uptime: 40 hours, 57 mins"));
                            app.messages.push(format!("             `/:-:++oooo+:               Packages: 12780 (pacman), 120 (flatpak)"));
                            app.messages.push(format!("            `/++++/+++++++:              Shell: Totally"));
                            app.messages.push(format!("           `/++++++++++++++:             Resolution: 25600x14400"));
                            app.messages.push(format!("          `/+++ooooooooooooo/`           WM: sway"));
                            app.messages.push(format!("         ./ooosssso++osssssso+`          Theme: Adwaita [GTK2/3]"));
                            app.messages.push(format!("        .oossssso-````/ossssss+`         Icons: breeze-dark [GTK2/3]"));
                            app.messages.push(format!("       -osssssso.      :ssssssso.        Terminal: It's complicated"));
                            app.messages.push(format!("      :osssssss/        osssso+++.       Terminal Font: Inconsolata"));
                            app.messages.push(format!("     /ossssssss/        +ssssooo/-       CPU: Intel i7-6600U (4) @ 30.400GHz"));
                            app.messages.push(format!("   `/ossssso+/:-        -:/+osssso+-     GPU: Intel Skylake GT2 [HD Graphics]"));
                            app.messages.push(format!("  `+sso+:-`                 `.-/+oso:    Memory: 2MiB / 118290GiB"));
                            app.messages.push(format!(" `++:.                           `-/+/"));
                            app.messages.push(format!(" .`                                 `/"));
                      };
                        app.messages.push(app.input.drain(..).collect());
                        if output != "" {
                            app.messages.push(output);
                        };
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn handleinput(app: &mut App) -> String {
    let msg = app.input.clone();
    let mut output = format!("empty");
    if app.state < 4 {
        let trimmed = msg.trim();
        match trimmed.parse::<f32>() {
            Ok(i) => {
                app.parameters[app.state] = i;
            }
            Err(..) => {app.messages.push("Not a number".to_string())}
        };
    };
    match app.state {
         0 => {
            output = format!("input maximum dose");
        },
         1 => {
            output = format!("input amount inside item");
        },
        _ => {},
    };
    if app.state >= 2 {
        let max_dose = calculate_dose(app.parameters[0], app.parameters[1]);
        let max_items = calculate_max_items(max_dose, app.parameters[2]);
        output = format!("Safe amount of these is {max_items}");
        app.is_on = false;
    };
    return output
}

