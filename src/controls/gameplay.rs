use std::{thread, time};
use crossterm::{
    cursor::{MoveTo, Hide},
    event::{self, Event, KeyCode},
    terminal::{self, enable_raw_mode, disable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};

use crate::{controls::utils::{burrito, cuatro_burritos, tres_burritos}, print_ascii};

const TEN_SECOND: time::Duration = time::Duration::from_millis(2500);
const _ONE_SECOND: time::Duration = time::Duration::from_millis(1000);
const NUMBER_CLICKS: usize = 100;

pub const PLAYER1: char = 'd';
pub const PLAYER2: char = 'l';
pub const PLAYER3: char = 'g';
pub const PLAYER4: char = 'h';

pub fn gameplay_1(){
    let mut input_from_player_1:usize = 0;
    let mut input_from_player_2:usize = 0;

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout.execute(terminal::DisableLineWrap).unwrap();
    
    stdout.execute(Hide).unwrap();use std::io;

    println!("\x1B[2J\x1B[1;1H");
    println!("Gameplay 1");
    println!("Player 1: Press {}", PLAYER1.to_ascii_uppercase());
    println!("Player 2: Press {}", PLAYER2.to_ascii_uppercase());

    thread::sleep(TEN_SECOND);

    // print_ascii::menu::load_game();

    loop {
        enable_raw_mode().unwrap();
        stdout.execute(MoveTo(0, 0)).unwrap();

        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char(PLAYER1) => {
                        input_from_player_1 += 1;
                    }
                    KeyCode::Char(PLAYER2) => {
                        input_from_player_2 += 1;
                    }
                    _ => {}
                }

                stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
                disable_raw_mode().unwrap();
                burrito(input_from_player_1, input_from_player_2);
            }
        }

        if input_from_player_1 == NUMBER_CLICKS || input_from_player_2 == NUMBER_CLICKS { break; }
    }

    enable_raw_mode().unwrap();
    stdout.execute(MoveTo(0, 0)).unwrap();

    if input_from_player_1 > input_from_player_2 {
        println!("\x1B[2J\x1B[1;1H");
        println!("Player 1 won!!");
    }
    else if  input_from_player_1 == input_from_player_2{
        println!("\x1B[2J\x1B[1;1H");
        println!("TIE!!!");
    } else {
        println!("\x1B[2J\x1B[1;1H");
        println!("Player 2 won!!");
    }
    thread::sleep(TEN_SECOND);
}

pub fn gameplay_2(){
    let mut input_from_player_1:usize = 0;
    let mut input_from_player_2:usize = 0;
    let mut input_from_player_3:usize = 0;

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout.execute(terminal::DisableLineWrap).unwrap();
    
    stdout.execute(Hide).unwrap();use std::io;

    println!("\x1B[2J\x1B[1;1H");
    println!("Gameplay 2");
    println!("Player 1: Press {}", PLAYER1.to_ascii_uppercase());
    println!("Player 2: Press {}", PLAYER2.to_ascii_uppercase());
    println!("Player 3: Press {}", PLAYER3.to_ascii_uppercase());

    loop {
        enable_raw_mode().unwrap();
        stdout.execute(MoveTo(0, 0)).unwrap();

        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char(PLAYER1) => {
                        input_from_player_1 += 1;
                    }
                    KeyCode::Char(PLAYER2) => {
                        input_from_player_2 += 1;
                    }
                    KeyCode::Char(PLAYER3) => {
                        input_from_player_3 += 1;
                    }
                    _ => {}
                }

                stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
                disable_raw_mode().unwrap();
                tres_burritos(input_from_player_1, input_from_player_2, input_from_player_3);
            }
        }

        if input_from_player_1 == NUMBER_CLICKS || input_from_player_2 == NUMBER_CLICKS || input_from_player_3 == NUMBER_CLICKS { break; }
    }

    let lock_list = vec![input_from_player_1, input_from_player_2, input_from_player_3];
    let mut list = vec![input_from_player_1, input_from_player_2, input_from_player_3];
    list.sort();
    list.reverse();
    enable_raw_mode().unwrap();
    stdout.execute(MoveTo(0, 0)).unwrap();

    println!("\x1B[2J\x1B[1;1H");
    for i in 0..lock_list.len() {
        if lock_list[i] == list[0]{
            println!("Player {} won!!", i+1);
            break;
        }
    }

    thread::sleep(TEN_SECOND);
}

pub fn gameplay_3(){
    let mut input_from_player_1:usize = 0;
    let mut input_from_player_2:usize = 0;
    let mut input_from_player_3:usize = 0;
    let mut input_from_player_4:usize = 0;

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout.execute(terminal::DisableLineWrap).unwrap();
    
    stdout.execute(Hide).unwrap();use std::io;

    println!("\x1B[2J\x1B[1;1H");
    println!("Gameplay 3");
    println!("Player 1: Press {}", PLAYER1.to_ascii_uppercase());
    println!("Player 2: Press {}", PLAYER2.to_ascii_lowercase());
    println!("Player 3: Press {}", PLAYER3.to_ascii_uppercase());
    println!("Player 4: Press {}", PLAYER4.to_ascii_uppercase());

    loop {
        enable_raw_mode().unwrap();
        stdout.execute(MoveTo(0, 0)).unwrap();

        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char(PLAYER1) => {
                        input_from_player_1 += 1;
                    }
                    KeyCode::Char(PLAYER2) => {
                        input_from_player_2 += 1;
                    }
                    KeyCode::Char(PLAYER3) => {
                        input_from_player_3 += 1;
                    }
                    KeyCode::Char(PLAYER4) => {
                        input_from_player_4 += 1;
                    }
                    _ => {}
                }

                stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
                disable_raw_mode().unwrap();
                cuatro_burritos(input_from_player_1, input_from_player_2, 
                    input_from_player_3, input_from_player_4);
            }
        }

        if input_from_player_1 == NUMBER_CLICKS || input_from_player_2 == NUMBER_CLICKS 
        || input_from_player_3 == NUMBER_CLICKS || input_from_player_4 == NUMBER_CLICKS { break; }
    }

    let lock_list = vec![input_from_player_1, input_from_player_2, input_from_player_3, input_from_player_4];
    let mut list = vec![input_from_player_1, input_from_player_2, input_from_player_3, input_from_player_4];
    list.sort();
    list.reverse();
    enable_raw_mode().unwrap();
    stdout.execute(MoveTo(0, 0)).unwrap();

    println!("\x1B[2J\x1B[1;1H");
    for i in 0..lock_list.len() {
        if lock_list[i] == list[0]{
            println!("Player {} won!!", i+1);
            break;
        }
    }

    thread::sleep(TEN_SECOND);
}

pub fn controls(){
    const MAX_CONTROLES:usize = 5;
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout.execute(terminal::DisableLineWrap).unwrap();
    
    stdout.execute(Hide).unwrap();use std::io;

    let mut cont = 0;
    let mut flag = false;
    loop {
        if cont == 0 {
            stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
            disable_raw_mode().unwrap();
            print_ascii::controls::controls_interface(cont);
        }
        enable_raw_mode().unwrap();
        stdout.execute(MoveTo(0, 0)).unwrap();

        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Down => {
                        if cont < MAX_CONTROLES {
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
                            cont = MAX_CONTROLES;
                        }
                    }
                    KeyCode::Esc => {
                        println!("\x1B[2J\x1B[1;1H");
                        println!("Exiting...");
                        break;
                    }
                    KeyCode::Enter =>{
                        flag = true;
                    }
                    _ => {}
                }

                if flag == true {
                    if cont == 5 {
                        println!("\x1B[2J\x1B[1;1H");
                        println!("Exiting...");
                        break;
                    }
                } else {
                    stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
                    disable_raw_mode().unwrap();
                    println!("\x1B[2J\x1B[1;1H");
                    print_ascii::controls::controls_interface(cont-1);
                }
            }
        }
    }
}

pub fn dda(){
    print!("NO HACE NADA ESTA FUNCION XDD");
}