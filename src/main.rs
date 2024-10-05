use std::io::{self, Write}; // Import Write trait

fn main() {
    //initialization
    let rows =8;
    let cols =8;
    let mut grid:  Vec<Vec<char>>  = vec![vec!['.'; cols]; rows];
    grid[3][3]='W';
    grid[4][4]='W';
    grid[3][4]='B';
    grid[4][3]='B';
    let mut current_player = 'B';
    let mut end_game=false;
    display(&grid);
    

    while !end_game{
        //get input until it is valid input
        loop {
            let mut input = String::new();   
            // Prompt the user for input
            print!("Enter move for colour {} (RowCol): ", current_player);
            // Flush the output buffer to ensure the prompt is displayed
            io::stdout().flush().unwrap();
    
            // Read the input 
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
    
            // Try to parse the input as a u32
            match make_move(&input, &mut grid, &mut current_player) {
                Ok(_) => {
                    break; // Exit on valid input
                }
                Err(_) => {
                    println!("Invalid move. Try again.");
                    display(&grid);
                }
            }

        }
        display(&grid);
        
        if has_valid_move(&grid, &mut opposite(&mut current_player)) {
            current_player = opposite(&mut current_player);
        }else if ! has_valid_move(&grid, &mut current_player) {
            let mut count_b = 0;
            let mut count_w = 0;
            for row in &grid {
                for val in row {
                    if *val=='B'{
                        count_b +=1;
                    }else if *val =='W'{
                        count_w +=1;
                    }
                }
            }
            if count_b > count_w{
                println!("Black wins by {} points!", count_b-count_w);
            }else if count_b == count_w {
                println!("Draw!");
            }else{
                println!("White wins by {} points!", count_w-count_b);
            }
            end_game=true;
        }
    }
}
 

fn display(grid_board: &Vec<Vec<char>>){
    let s = String::from("  abcdefgh");
    println!("{s}");

    for (index,value) in grid_board.iter().enumerate() {
        println!("{} {}",((97+index as u8) as char), value.iter().collect::<String>());
    }
}


fn opposite(current_player: &mut char)  -> char{
    if *current_player == 'W' {
        return 'B';
    }else{
        return 'W';
    }
}


fn has_valid_move(grid_board: &Vec<Vec<char>>,  current_player: &mut char) -> bool {
    let directions = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),         (0, 1),
    (1, -1), (1, 0), (1, 1),
    ];
    for i in 0..grid_board.len() {
        for j in 0..grid_board[i].len() {
            if grid_board[i][j]=='.' {
                for &(dr, dc) in &directions {
                    let mut r = i as isize + dr;
                    let mut c = j as isize + dc;
                    let mut has_opponent_piece = false;
            
                    while r >= 0 && r < 8 && c >= 0 && c < 8 {
                        match grid_board[r as usize][c as usize] {
                            p if p == opposite(current_player) => {
                                has_opponent_piece = true;
                            },
                            p if p == *current_player && has_opponent_piece => {
                                return true
                            },
                            _ => break,
                        }
                        //move further on the same direction until find current_player's pos
                        r += dr; 
                        c += dc;
                    }
            
                }           
            }
        }
    }
    println!("{} player has no valid move.", current_player);
    return false;
}

fn make_move(input: &str, grid_board: &mut Vec<Vec<char>>, current_player: &mut char) -> Result<(), &'static str> {
    let input_trim = &input.trim();


    if input_trim.len() != 2 {
        return Err("Invalid input length.Please input 2 small characters");
    }

    //must be all small characters
    if !input_trim.chars().all(|c| c.is_ascii_lowercase()) {
        return Err("Invalid input pos characters.Please input 2 small characters");
    }

    //already checked in previous steps, so directly call unwrap()
    //grid[r][c], r and c should be casted to usize
    let row = (input_trim.chars().nth(0).unwrap() as usize)-97;
    let col = (input_trim.chars().nth(1).unwrap() as usize)-97;
    if row>=8 || col>=8 || !(grid_board[row][col]=='.'){
        return Err("Occupied pos or out of range");
    }

    //check all 8 directions to make sure it besides opponent 
    //if so, immediately do the flip
    let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),         (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];

    let curr : char = *current_player;
    let mut valid_move = false;
    for &(dr, dc) in &directions {
        let mut r = row as isize + dr;
        let mut c = col as isize + dc;
        let mut has_opponent_piece = false;

        let mut r0 = row as isize;
        let mut c0 = col as isize;

        while r >= 0 && r < 8 && c >= 0 && c < 8 {
            match grid_board[r as usize][c as usize] {
                p if p == opposite(current_player) => {
                    has_opponent_piece = true;
                },
                //current_player is &mut String
                // unwrap() : Option<char> to char
                p if p == *current_player && has_opponent_piece => {
                    //valid_mooth=true;
                    //validation goes successfully
                    //return true
                    //return Ok(())
                    valid_move=true;
                    while r0!=r || c0!=c {
                        grid_board[r0 as usize][c0 as usize]=curr;
                        r0 += dr;
                        c0 += dc;
                    }
                    break;           
                },
                _ => break,
            }
            //move further on the same direction until find current_player's pos
            r += dr; 
            c += dc;
        }

    }

    //validation fail
    if valid_move{
        return Ok(())
    }
    return Err("No opponent piece at any of the 8 directions or no valid straight line.");
    
}
