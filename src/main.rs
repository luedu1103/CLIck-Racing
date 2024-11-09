use crossterm::{
    cursor::{MoveTo, Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, enable_raw_mode, disable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};

mod print_ascii;
mod controls;

const MAX_MENUS:usize = 5;

fn main() {
    main_loop();
}

fn main_loop() {
    let game_selection: Vec<fn()> = vec![
        controls::gameplay::dda,
        controls::gameplay::gameplay_1,
        controls::gameplay::gameplay_2,
        controls::gameplay::gameplay_3,
        controls::gameplay::controls,
    ];

    let mut cont: usize = 0;
    let mut game_control: Option<usize> = None;

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    
    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout.execute(terminal::DisableLineWrap).unwrap();
    
    stdout.execute(Hide).unwrap();use std::io;
    
    loop {
        if cont == 0 {
            stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
            disable_raw_mode().unwrap();
            print_ascii::menu::menus_interface(cont);
        }

        enable_raw_mode().unwrap();
        stdout.execute(MoveTo(0, 0)).unwrap();
        
        if event::poll(std::time::Duration::from_millis(500)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Down => {
                        // Wrap around to the last item if at the top
                        if cont < MAX_MENUS {
                            cont += 1;
                        } else {
                            cont = 1;
                        }
                    }
                    KeyCode::Up => {
                        // Wrap around to the last item if at the top
                        if cont > 1 {
                            cont -= 1;
                        } else {
                            cont = MAX_MENUS;
                        }
                    }
                    KeyCode::Esc => {
                        println!("\x1B[2J\x1B[1;1H");
                        println!("Exiting...");
                        break;
                    }
                    KeyCode::Enter =>{
                        game_control = Some(cont);
                    }
                    _ => {}
                }

                if game_control != None {
                    if game_control.unwrap() == 5{
                        println!("\x1B[2J\x1B[1;1H");
                        println!("Exiting...");
                        break;
                    }
                    stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
                    disable_raw_mode().unwrap();
                    println!("\x1B[2J\x1B[1;1H");
                    game_selection[game_control.unwrap()]();
                    game_control = None;
                    cont = 0;
                } else {
                    // Clear current line and call corresponding menu function
                    stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
                    disable_raw_mode().unwrap();
                    if cont > 0 {
                        print_ascii::menu::menus_interface(cont-1);
                    }
                }
            }
        }
    }

    // Clean up: show cursor and disable raw mode before exiting
    stdout.execute(Show).unwrap();
    disable_raw_mode().unwrap();
}
