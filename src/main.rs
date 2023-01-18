use std::process::Command;
use std::thread::{self,sleep};
use signal_hook::{consts::SIGINT, iterator::Signals};

fn main() {
    
    handl_ctrlc();

    new_buffer();
    clear_screen();
    println!("Hello, world!");
    let mut buffer = String::new();
    let x = std::io::stdin().read_line(&mut buffer).unwrap_or(0);

    exit_buffer(true);
    dbg!(x);
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

