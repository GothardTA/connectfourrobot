use std::process::{Command, CommandArgs};
use rand::Rng;
use image::{io::Reader as ImageReader, Pixel};

fn main() {
    println!("Connect Four Robot v0.1.0 - MIT license - see https://github.com/GothardTA/connectfourrobot for more details");

    // calls the pi's libcamera-still command to take a picture and save it to a file
    let output = Command::new("bash").arg("takepic.sh").output().expect("Failed to take picture");
    // Command::new("sleep").arg("5").output().expect("Failed to sleep");
    let msg = match String::from_utf8(output.stdout) {
        Ok(v) => v,
        Err(e) => String::from("Error".to_owned() + &e.to_string()),
    };
    println!("{}", msg);
    println!("Picture Captured and saved as image.jpg");

    let img = ImageReader::open("image.jpg").expect("Failed to open file").decode().expect("Failed to decode image").into_rgba8();
    
    let positions = [
        [[80, 340], [150, 400], [206, 414], [252, 420], [286, 426], [317, 436], [341, 434]],
        [[116, 297], [182, 322], [230, 340], [275, 355], [306, 370], [335, 380], [360, 390]],
        [[150, 213], [214, 243], [260, 270], [300, 300], [327, 311], [353, 328], [374, 344]],
        [[190, 140], [244, 178], [285, 213], [326, 240], [350, 260], [370, 276], [392, 294]],
        [[222, 62], [272, 112], [313, 148], [344, 184], [366, 207], [390, 240], [406, 248]],
        [[260, 9], [303, 46], [333, 93], [360, 126], [386, 164], [404, 186], [422, 207]]
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
    println!("{:#?}", board);
    println!("{}", ai_move(&mut board, 'Y'))
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