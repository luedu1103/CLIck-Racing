use std::{thread, time};

pub fn menus_interface(arrow: usize){
    let mut arrows:Vec<Option<String>> = vec![None, None, None, None, None];
    arrows[arrow] = Some("->".to_string());
    println!("\x1B[2J\x1B[1;1H");
    println!(r"
 _______________________________________________________
|      ____             _             ________    ____  |
|     / __ \____ ______(_)___  ____ _/ ____/ /   /  _/  |
|    / /_/ / __ `/ ___/ / __ \/ __ `/ /   / /    / /    |
|   / _, _/ /_/ / /__/ / / / / /_/ / /___/ /____/ /     |
|  /_/ |_|\__,_/\___/_/_/ /_/\__, /\____/_____/___/     |
|                           /____/                      |
|                                                       |
|                     {} 2 players                      |
|                     {} 3 players                      |
|                     {} 4 players                      |
|                     {} Controls                       |
|                     {} Exit                           |
|_______________________________________________________|",
arrows[0].clone().unwrap_or("  ".to_string()),
arrows[1].clone().unwrap_or("  ".to_string()),
arrows[2].clone().unwrap_or("  ".to_string()),
arrows[3].clone().unwrap_or("  ".to_string()),
arrows[4].clone().unwrap_or("  ".to_string()));
}

pub fn _load_game() {
    for i in (0..5).rev() {
        let one_second = time::Duration::from_millis(1000);
        println!("\x1B[2J\x1B[1;1H");
        println!("{}", i);
        if i == 0 {
            println!("\x1B[2J\x1B[1;1H");
            println!("Start!");
        }
        thread::sleep(one_second);    
    }
}