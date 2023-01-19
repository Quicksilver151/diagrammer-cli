use std::error::Error;
use std::{io, thread, time::Duration};
use std::process::Command;
use thread::sleep;
use signal_hook::{consts::SIGINT, iterator::Signals};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
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
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;
    
    thread::sleep(Duration::from_millis(5000));
    
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


