use std::error::Error;
use std::{io, thread, time::Duration};
use std::process::Command;
use thread::sleep;
use signal_hook::{consts::SIGINT, iterator::Signals};
use tui::style::Style;
use tui::{
    style::*,
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction, Alignment},
    Terminal,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(),std::io::Error>{
    handl_ctrlc();
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    terminal.draw(|f| {
        let mut size = f.size();
        
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL)
            // .border_style(Style { fg: Some(Color::Red), bg: None, add_modifier: Modifier::HIDDEN, sub_modifier: Modifier::RAPID_BLINK })
            // .style(Style { fg: Some(Color::Red), bg: None, add_modifier: Modifier::HIDDEN, sub_modifier: Modifier::RAPID_BLINK })
            .title_alignment(Alignment::Center);
        let border_factor = 1.5;
        
        let (og_width, og_height) = (size.width, size.height);
        size.width  = (size.width as f32/border_factor as f32).round() as u16;
        size.height = (size.height as f32/border_factor as f32).round() as u16;
        
        size.x = (og_width  - size.width)/2;
        size.y = (og_height - size.height)/2;
        
        // dbg!(size);
        f.render_widget(block, size);
    })?;
    
    thread::sleep(Duration::from_millis(1000));
    
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    Ok(())
    
    
    //
    // new_buffer();
    // clear_screen();
    // println!("Hello, world!");
    // let mut buffer = String::new();
    // let x = std::io::stdin().read_line(&mut buffer).unwrap_or(0);
    //
    // exit_buffer(true);
    // dbg!(x);
}



// terminal screen functions
fn clear_screen(){
    print!("\x1B[2J");
    print!("\x1b[1;1H");
}
fn new_buffer(){
    print!("\x1b[?1049h");
}
fn exit_buffer(new_line:bool){
    print!("\x1b[?1049l");
    if new_line{println!()}
}

// handle SIGINT
fn handl_ctrlc(){
    let mut signals = Signals::new([SIGINT]).unwrap();
    
    thread::spawn(move || {
        for sig in signals.wait() {
            if sig == 2{exit_buffer(false);std::process::exit(0)}
        }
    });
}

// notifications
// fn notify_send(message:&str){
//     Command::new("notify-send").args(["--urgency=critical", message]).output().expect("failed");
// }


