pub fn burrito (number1: usize, number2: usize) {
    let mut space1 = String::from("");
    let mut space2 = String::from("");
    for _ in 0..number1 {
        space1 += " ";
    }

    for _ in 0..number2 {
        space2 += " ";
    }
    println!("\x1B[2J\x1B[1;1H");
    println!(r"
    {}Player 1
    {}______
    {}|     |\
    {}|     |_\
    {}|_______|
    
    {}Player 2
    {}______
    {}|     |\
    {}|     |_\
    {}|_______|
    ", space1,space1, space1, space1, space1, space2, space2, space2, space2, space2);
}

pub fn cuatro_burritos (number1: usize, number2: usize, number3: usize, number4: usize) {
    let mut space1 = String::from("");
    let mut space2 = String::from("");
    let mut space3 = String::from("");
    let mut space4 = String::from("");
    for _ in 0..number1 {
        space1 += " ";
    }

    for _ in 0..number2 {
        space2 += " ";
    }

    for _ in 0..number3 {
        space3 += " ";
    }

    for _ in 0..number4 {
        space4 += " ";
    }

    println!("\x1B[2J\x1B[1;1H");
    println!(r"
    {}Player 1
    {}______
    {}|     |\
    {}|     |_\
    {}|_______|
    
    {}Player 2
    {}______
    {}|     |\
    {}|     |_\
    {}|_______|

    {}Player 3
    {}______
    {}|     |\
    {}|     |_\
    {}|_______|

    {}Player 4
    {}______
    {}|     |\
    {}|     |_\
    {}|_______|
    ", space1,space1, space1, space1, space1, 
    space2, space2, space2, space2, space2, 
    space3, space3, space3, space3, space3, 
    space4, space4, space4, space4, space4);
}

pub fn tres_burritos (number1: usize, number2: usize, number3: usize) {
    let mut space1 = String::from("");
    let mut space2 = String::from("");
    let mut space3 = String::from("");
    for _ in 0..number1 {
        space1 += " ";
    }

    for _ in 0..number2 {
        space2 += " ";
    }

    for _ in 0..number3 {
        space3 += " ";
    }

    println!("\x1B[2J\x1B[1;1H");
    println!(r"
    {}Player 1
    {}______
    {}|     |\
    {}|     |_\
    {}|_______|
    
    {}Player 2
    {}______
    {}|     |\
    {}|     |_\
    {}|_______|

    {}Player 3
    {}______
    {}|     |\
    {}|     |_\
    {}|_______|
    ", space1,space1, space1, space1, space1, 
    space2, space2, space2, space2, space2, 
    space3, space3, space3, space3, space3);
}