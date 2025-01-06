use crate::constants::controls_key;

pub fn controls_interface(arrow: usize){
    let mut arrows:Vec<Option<String>> = vec![None, None, None, None, None];
    arrows[arrow] = Some("-> ".to_string());
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
|                  {} Player 1 key: {}                  |
|                  {} Player 2 key: {}                  |
|                  {} Player 3 key: {}                  |
|                  {} Player 4 key: {}                  |
|                       {} Exit                        |
|_______________________________________________________|", 
arrows[0].clone().unwrap_or("   ".to_string()), controls_key::PLAYER1, 
arrows[1].clone().unwrap_or("   ".to_string()), controls_key::PLAYER2, 
arrows[2].clone().unwrap_or("   ".to_string()), controls_key::PLAYER3, 
arrows[3].clone().unwrap_or("   ".to_string()), controls_key::PLAYER4,
arrows[4].clone().unwrap_or("   ".to_string()));
}