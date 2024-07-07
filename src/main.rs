use rand::Rng;
use image::{io::Reader as ImageReader, Pixel};
use std::{process::Command, thread, time::Duration};
use gpio::{GpioOut, GpioIn};

fn main() {
    println!("Connect Four Robot v0.1.0 - MIT license - see https://github.com/GothardTA/connectfourrobot for more details");

    let mut gpio23 = gpio::sysfs::SysFsGpioOutput::open(23).unwrap();
    let mut gpio24 = gpio::sysfs::SysFsGpioOutput::open(24).unwrap();
    let mut gpio25 = gpio::sysfs::SysFsGpioOutput::open(25).unwrap();
    let mut gpio12 = gpio::sysfs::SysFsGpioInput::open(12).unwrap();

    // calls the pi's libcamera-still command to take a picture and save it to a file
    let output = Command::new("bash").arg("takepic.sh").output().expect("Failed to take picture");
    
    let msg = match String::from_utf8(output.stdout) {
        Ok(v) => v,
        Err(e) => String::from("Error".to_owned() + &e.to_string()),
    };
    println!("{}", msg);
    println!("Picture Captured and saved as image.jpg");

    let img = ImageReader::open("image.jpg").expect("Failed to open file").decode().expect("Failed to decode image").into_rgba8();
    
    let positions = [
        [[24, 714], [141, 725], [237, 730], [320, 718], [380, 721], [434, 717], [482, 716]],
        [[90, 532], [195, 571], [284, 585], [353, 601], [411, 612], [459, 616], [502, 626]],
        [[138, 390], [241, 430], [320, 463], [376, 499], [435, 513], [476, 535], [519, 547]],
        [[181, 253], [273, 307], [350, 354], [408, 391], [455, 428], [498, 445], [537, 469]],
        [[222, 105], [307, 190], [378, 249], [434, 283], [475, 342], [515, 368], [553, 385]],
        [[257, 13], [340, 84], [402, 160], [453, 212], [503, 258], [535, 284], [575, 320]]
    ];

    let mut board: [[u8; 7]; 6] = [[0; 7]; 6];

    for row in 0..positions.len() {
        for col in 0..positions[0].len() {
            let rg_difference = 
                (img.get_pixel(positions[row][col][0], positions[row][col][1]).to_rgb()[0] as i32 -
                img.get_pixel(positions[row][col][0], positions[row][col][1]).to_rgb()[1] as i32).abs();
            
            let rb_difference = 
                (img.get_pixel(positions[row][col][0], positions[row][col][1]).to_rgb()[0] as i32 -
                img.get_pixel(positions[row][col][0], positions[row][col][1]).to_rgb()[2] as i32).abs();

            let gb_difference = 
                (img.get_pixel(positions[row][col][0], positions[row][col][1]).to_rgb()[1] as i32 -
                img.get_pixel(positions[row][col][0], positions[row][col][1]).to_rgb()[2] as i32).abs();
            
            if rg_difference > 40 && rb_difference > 40 && gb_difference <= 80 {
                board[row][col] = b'R';
            } else if rg_difference <= 40 && rb_difference > 40 && gb_difference > 40 {
                board[row][col] = b'Y';
            } else {
                board[row][col] = b' ';
            }
        }
    }

    display_board(&board);

    for _iter in 0..6 {
        for col in 0..7 {
            for row in 0..5 {
                if board[row][col] != b' ' && board[row + 1][col] == b' ' {
                    board[row][col] = b' ';
                }
            }
        }
    }

    display_board(&board);
    let col = ai_move(&mut board, 'Y');
    println!("{}", col + 1);

    if col == 0 {
        gpio23.set_value(false).expect("could not set gpio23");
        gpio25.set_value(false).expect("could not set gpio25");
        gpio24.set_value(true).expect("could not set gpio24");
    } else if col == 1 {
        gpio23.set_value(false).expect("could not set gpio23");
        gpio25.set_value(true).expect("could not set gpio25");
        gpio24.set_value(false).expect("could not set gpio24");
    } else if col == 2 {
        gpio23.set_value(false).expect("could not set gpio23");
        gpio25.set_value(true).expect("could not set gpio25");
        gpio24.set_value(true).expect("could not set gpio24");
    } else if col == 3 {
        gpio23.set_value(true).expect("could not set gpio23");
        gpio25.set_value(false).expect("could not set gpio25");
        gpio24.set_value(false).expect("could not set gpio24");
    } else if col == 4 {
        gpio23.set_value(true).expect("could not set gpio23");
        gpio25.set_value(false).expect("could not set gpio25");
        gpio24.set_value(true).expect("could not set gpio24");
    } else if col == 5 {
        gpio23.set_value(true).expect("could not set gpio23");
        gpio25.set_value(true).expect("could not set gpio25");
        gpio24.set_value(false).expect("could not set gpio24");
    } else if col == 6 {
        gpio23.set_value(true).expect("could not set gpio23");
        gpio25.set_value(true).expect("could not set gpio25");
        gpio24.set_value(true).expect("could not set gpio24");
    }

    thread::sleep(Duration::from_millis(1000));
    gpio23.set_value(false).expect("could not set gpio23");
    gpio25.set_value(false).expect("could not set gpio25");
    gpio24.set_value(false).expect("could not set gpio24");
    
    println!("Sent value over cable");
}

// outputs the board to the screen
fn display_board(board: &[[u8; 7]; 6]) {
    // print!("{}[2J", 27 as char);
    for row in board {
        for spot in row {
            if *spot == b'R' {
                print!("|\x1b[41m {} \x1b[0m", *spot as char);
            } else if *spot == b'Y' {
                print!("|\x1b[43m {} \x1b[0m", *spot as char);
            } else {
                print!("|\x1b[44m {} \x1b[0m", *spot as char);
            }
        }
        println!("|");
        for _i in 0..row.len() {
            print!("|___");
        }
        println!("|");
    }
    println!("  1   2   3   4   5   6   7");
}

fn play_move(board: &mut [[u8; 7]; 6], col: usize, player: char) -> bool {
    let mut lowest_row: i8 = -1;
    for i in 0..6 {
        if board[i][col] == b' ' {
            lowest_row += 1;
        } else {
            break;
        }
    }

    if lowest_row == -1 {
        return false;
    }

    if board[lowest_row as usize][col] != b' ' {
        return false;
    }

    board[lowest_row as usize][col] = player as u8;
    return true;
}

// checks the board for any four in a row and return the color (R, Y) that won
fn check_four_in_a_row(board: &[[u8; 7]; 6], player: char) -> bool {
    // vertical four in a row check
    for row in 0..(board.len()-3) {
        for col in 0..(board[row].len()) {
            if
                board[row][col] == board[row+1][col] &&
                board[row+1][col] == board[row+2][col] &&
                board[row+2][col] == board[row+3][col]
            {
                if board[row][col] == player as u8 {
                    return true;
                }
            }
        }
    }

    // horizontal four in a row check
    for row in 0..(board.len()) {
        for col in 0..(board[row].len()-3) {
            if
                board[row][col] == board[row][col+1] &&
                board[row][col+1] == board[row][col+2] &&
                board[row][col+2] == board[row][col+3]
            {
                if board[row][col] == player as u8 {
                    return true;
                }
            }
        }
    }

    // diagonal left to right four in a row check
    for row in 0..(board.len()-3) {
        for col in 0..(board[row].len()-3) {
            if
                board[row][col] == board[row+1][col+1] &&
                board[row+1][col+1] == board[row+2][col+2] &&
                board[row+2][col+2] == board[row+3][col+3]
            {
                if board[row][col] == player as u8 {
                    return true;
                }
            }
        }
    }

    // diagonal right to left four in a row check
    for row in 0..(board.len()-3) {
        for col in 3..(board[row].len()) {
            if
                board[row][col] == board[row+1][col-1] &&
                board[row+1][col-1] == board[row+2][col-2] &&
                board[row+2][col-2] == board[row+3][col-3]
            {
                if board[row][col] == player as u8 {
                    return true;
                }
            }
        }
    }
    return false;
}

// checks the board for any four in a row and return the color (R, Y) that won
fn check_three_in_a_row(board: &[[u8; 7]; 6], player: char) -> bool {
    // vertical four in a row check
    for row in 0..(board.len()-2) {
        for col in 0..(board[row].len()) {
            if
                board[row][col] == board[row+1][col] &&
                board[row+1][col] == board[row+2][col]
            {
                if board[row][col] == player as u8 {
                    return true;
                }
            }
        }
    }

    // horizontal four in a row check
    for row in 0..(board.len()) {
        for col in 0..(board[row].len()-2) {
            if
                board[row][col] == board[row][col+1] &&
                board[row][col+1] == board[row][col+2]
            {
                if board[row][col] == player as u8 {
                    return true;
                }
            }
        }
    }

    // diagonal left to right four in a row check
    for row in 0..(board.len()-2) {
        for col in 0..(board[row].len()-2) {
            if
                board[row][col] == board[row+1][col+1] &&
                board[row+1][col+1] == board[row+2][col+2]
            {
                if board[row][col] == player as u8 {
                    return true;
                }
            }
        }
    }

    // diagonal right to left four in a row check
    for row in 0..(board.len()-2) {
        for col in 2..(board[row].len()) {
            if
                board[row][col] == board[row+1][col-1] &&
                board[row+1][col-1] == board[row+2][col-2]
            {
                if board[row][col] == player as u8 {
                    return true;
                }
            }
        }
    }
    return false;
}

// checks the board for any four in a row and return the color (R, Y) that won
fn check_two_in_a_row(board: &[[u8; 7]; 6], player: char) -> bool {
    // vertical four in a row check
    for row in 0..(board.len()-1) {
        for col in 0..(board[row].len()) {
            if
                board[row][col] == board[row+1][col]
            {
                if board[row][col] == player as u8 {
                    return true;
                }
            }
        }
    }

    // horizontal four in a row check
    for row in 0..(board.len()) {
        for col in 0..(board[row].len()-1) {
            if
                board[row][col] == board[row][col+1]
            {
                if board[row][col] == player as u8 {
                    return true;
                }
            }
        }
    }

    // diagonal left to right four in a row check
    for row in 0..(board.len()-1) {
        for col in 0..(board[row].len()-1) {
            if
                board[row][col] == board[row+1][col+1]
            {
                if board[row][col] == player as u8 {
                    return true;
                }
            }
        }
    }

    // diagonal right to left four in a row check
    for row in 0..(board.len()-1) {
        for col in 1..(board[row].len()) {
            if
                board[row][col] == board[row+1][col-1]
            {
                if board[row][col] == player as u8 {
                    return true;
                }
            }
        }
    }
    return false;
}

fn ai_move(board: &mut [[u8; 7]; 6], player: char) -> u8 {
    let mut tmp_board: [[u8; 7]; 6];
    let other_player: char;

    if player == 'R' {
        other_player = 'Y';
    } else {
        other_player = 'R';
    }

    // checks if the ai can win
    for col in 0..7 {
        tmp_board = board.clone();
        if !play_move(&mut tmp_board, col, player) {
            continue;
        }

        if check_four_in_a_row(&tmp_board, player) {
            return col as u8;
        }
    }

    // checks if the other player could win and block it
    for col in 0..7 {
        tmp_board = board.clone();
        if !play_move(&mut tmp_board, col, other_player) {
            continue;
        }

        if check_four_in_a_row(&tmp_board, other_player) {
            return col as u8;
        }
    }

    // checks if the ai can get a three in a row
    for col in 0..7 {
        tmp_board = board.clone();
        if !play_move(&mut tmp_board, col, player) {
            continue;
        }

        if check_three_in_a_row(&tmp_board, player) {
            return col as u8;
        }
    }

    // checks if the ai can get a two in a row
    for col in 0..7 {
        tmp_board = board.clone();
        if !play_move(&mut tmp_board, col, player) {
            continue;
        }

        if check_two_in_a_row(&tmp_board, player) {
            return col as u8;
        }
    }

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..7);
    return num;
}