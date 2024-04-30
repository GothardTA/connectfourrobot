use std::process::Command;
use rand::Rng;
use image::{io::Reader as ImageReader, Pixel};

fn main() {
    println!("Connect Four Robot v0.1.0 - MIT license - see https://github.com/GothardTA/connectfourrobot for more details");

    // calls the pi's libcamera-still command to take a picture and save it to a file
    Command::new("libcamera-still").arg("-n").arg("-t 1").arg("--width 640").arg("--height 480").arg("-o image.jpg").output().expect("Failed to take picture");

    let img = ImageReader::open("image.jpg").expect("Failed to open file").decode().expect("Failed to decode image").into_rgba8();
    
    let positions = [
        [[200, 75], [236, 75], [275, 75], [315, 75], [360, 75], [406, 75], [460, 75]],
        [[200, 110], [236, 110], [275, 110], [315, 110], [360, 110], [406, 110], [460, 110]],
        [[200, 145], [236, 145], [275, 145], [315, 145], [360, 145], [406, 145], [460, 145]],
        [[190, 190], [236, 190], [275, 190], [315, 190], [360, 190], [406, 190], [460, 190]],
        [[190, 230], [236, 230], [275, 230], [315, 230], [360, 240], [406, 240], [460, 240]],
        [[190, 270], [236, 270], [275, 270], [315, 270], [360, 290], [406, 290], [460, 290]]
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
            
            if rg_difference <= 40 && rb_difference <= 40 && gb_difference <= 40 {
                board[row][col] = b' ';
            } else if rg_difference > 50 && rb_difference > 50 && gb_difference <= 20 {
                board[row][col] = b'R';
            } else if rg_difference <= 40 && rb_difference > 50 && gb_difference > 50 {
                board[row][col] = b'Y';
            } else {
                println!("Row {}, Col {} failed to detect color", row, col);
            }
        }
    }
    println!("{:?}", board);
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

fn ai_move(board: &mut [[u8; 7]; 6], player: char) {
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
            play_move(board, col, player);
            return;
        }
    }

    // checks if the other player could win and block it
    for col in 0..7 {
        tmp_board = board.clone();
        if !play_move(&mut tmp_board, col, other_player) {
            continue;
        }

        if check_four_in_a_row(&tmp_board, other_player) {
            play_move(board, col, player);
            return;
        }
    }

    // checks if the ai can get a three in a row
    for col in 0..7 {
        tmp_board = board.clone();
        if !play_move(&mut tmp_board, col, player) {
            continue;
        }

        if check_three_in_a_row(&tmp_board, player) {
            play_move(board, col, player);
            return;
        }
    }

    // checks if the ai can get a two in a row
    for col in 0..7 {
        tmp_board = board.clone();
        if !play_move(&mut tmp_board, col, player) {
            continue;
        }

        if check_two_in_a_row(&tmp_board, player) {
            play_move(board, col, player);
            return;
        }
    }

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..7);
    play_move(board, num, player);
}