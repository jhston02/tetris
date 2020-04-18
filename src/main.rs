use std::io::{Write, stdout};
use crossterm::{execute, ExecutableCommand, cursor, QueueableCommand, style::Print, ErrorKind, terminal};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers, poll};
use std::path::Component::RootDir;
use std::{thread, time};
use rand::prelude::*;

const FIELD_WIDTH:usize = 12;
const FIELD_HIEGHT:usize = 18;

fn main() -> Result<(), ErrorKind> {
    let tetronimo1 = b"..X...X...X...X.";
    let tetronimo2 = b"..X..XX...X.....";
    let tetronimo3 = b".....XX..XX.....";
    let tetronimo4 = b".X...XX...X.....";
    let tetronimo5 = b".X...X...XX.....";
    let tetronimo6 = b"..X...X..XX.....";

    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let tetronimos = [tetronimo1, tetronimo2, tetronimo3, tetronimo4, tetronimo5, tetronimo6];
    let mut gameOver = false;


    let mut rng = rand::thread_rng();

    let mut game_board:[u8;216] = [0;216];

    for i in 0..FIELD_WIDTH {
        for j in 0..FIELD_HIEGHT {
            game_board[j*FIELD_WIDTH + i] = if i == 0 || i == FIELD_WIDTH - 1 || j == FIELD_HIEGHT - 1 {
                9
            }
            else {
                0
            };
        }
    }


    let mut should_render = true;
    let mut nCurrentPiece:u8 = rng.gen_range(0,6);;
    let mut rotation = 3;
    let mut nCurrentX:i32 = FIELD_WIDTH as i32 / 2;
    let mut nCurrentY: i32 = 0;
    let fifty_millis = time::Duration::from_millis(50);
    let mut speed:u8 = 20;
    let mut speedCounter:u8 = 0;
    let mut pieceCount = 0;

    //update_game_board(&mut game_board, tetronimos[nCurrentPiece as usize], nCurrentX, nCurrentY, &rotation, nCurrentPiece + 1);

    while(!gameOver) {
        thread::sleep(fifty_millis);
        speedCounter = speedCounter + 1;
        pieceCount += 1;
        if poll(time::Duration::from_millis(10))?
        {
            match read().unwrap() {
                //i think this speaks for itself
                Event::Key(KeyEvent {
                               code: KeyCode::Down,
                               modifiers: KeyModifiers::NONE,
                           }) => {
                    if does_piece_fit(tetronimos[nCurrentPiece as usize], rotation, nCurrentX, nCurrentY + 1, &game_board) {
                        nCurrentY += 1;
                        should_render = true;
                    }
                },
                Event::Key(KeyEvent {
                               code: KeyCode::Char('q'),
                               modifiers: KeyModifiers::NONE,
                           }) => break,
                Event::Key(KeyEvent {
                               code: KeyCode::Left,
                               modifiers: KeyModifiers::NONE,
                           }) => {
                    if does_piece_fit(tetronimos[nCurrentPiece as usize], rotation, nCurrentX - 1, nCurrentY, &game_board) {
                        nCurrentX -= 1;
                        should_render = true;
                    }
                },
                Event::Key(KeyEvent {
                               code: KeyCode::Right,
                               modifiers: KeyModifiers::NONE,
                           }) => {
                    if does_piece_fit(tetronimos[nCurrentPiece as usize], rotation, nCurrentX + 1, nCurrentY, &game_board) {
                        nCurrentX += 1;
                        should_render = true;
                    }
                },
                Event::Key(KeyEvent {
                               code: KeyCode::Char(' '),
                               modifiers: KeyModifiers::NONE,
                           }) => {
                    let tempRotation = (rotation + 1) % 4;
                    if does_piece_fit(tetronimos[nCurrentPiece as usize], tempRotation, nCurrentX , nCurrentY, &game_board) {
                        rotation = tempRotation;
                        should_render = true;
                    }
                },
                _ => (),
            }
        }

        if speedCounter >= speed {
            speedCounter = 0;
            if does_piece_fit(tetronimos[nCurrentPiece as usize], rotation, nCurrentX, nCurrentY + 1, &game_board) {
                nCurrentY += 1;
                should_render = true;
            }
            else {
                update_game_board(&mut game_board, tetronimos[nCurrentPiece as usize], nCurrentX, nCurrentY, rotation, nCurrentPiece + 1);

                if pieceCount % 10 == 0 {
                    speed -= 1;
                }

                let mut lines_list = Vec::new();

                for i in 0..4 {
                    if i + nCurrentY < (FIELD_HIEGHT as i32 - 1) {
                        let mut isLine = true;
                        for j in 1..(FIELD_WIDTH - 1) {
                            if game_board[((i as i32 + nCurrentY)*FIELD_WIDTH as i32 + (j as i32)) as usize] == 0 {
                                isLine = false;
                                break;
                            }
                        }
                        if isLine {
                            for j in 1..(FIELD_WIDTH - 1) {
                                game_board[((i as i32 + nCurrentY)*FIELD_WIDTH as i32 + j as i32 ) as usize] = 8;
                            }
                            let lines = get_render(&game_board, tetronimos[nCurrentPiece as usize], nCurrentX, nCurrentY, rotation, nCurrentPiece + 1, false);
                            stdout.queue(cursor::MoveTo(0, 0))?;
                            stdout.queue(Print(lines))?;
                            stdout.flush()?;
                            should_render = false;
                            lines_list.push(i + nCurrentY);
                            thread::sleep(time::Duration::from_millis(1000));

                        }
                    }
                }

                nCurrentPiece = rng.gen_range(0,6);

                rotation = 3;
                nCurrentX= FIELD_WIDTH as i32 / 2;
                nCurrentY = 0;
                should_render = true;

                for line in lines_list.iter() {
                    for i in 1..(FIELD_WIDTH - 1) {
                        let mut k = line.clone();
                        while k >0 {
                            game_board[((k) * FIELD_WIDTH as i32 + i as i32) as usize] = game_board[((k - 1 as i32) * FIELD_WIDTH as i32 + i as i32) as usize];
                            k -= 1;
                        }
                    }
                }

                if !does_piece_fit(tetronimos[nCurrentPiece as usize], rotation, nCurrentX, nCurrentY + 1, &game_board) {
                    gameOver = true;
                    should_render = true;
                }
            }
        }

        if should_render
        {
            let lines = get_render(&game_board, tetronimos[nCurrentPiece as usize], nCurrentX, nCurrentY, rotation, nCurrentPiece + 1, true);
            stdout.queue(cursor::MoveTo(0, 0))?;
            stdout.queue(Print(lines))?;
            stdout.flush()?;
            should_render = false;
        }
    }

    Ok(())
}

fn update_game_board(board:&mut [u8], tretronimo:&[u8;16], posX:i32, posY:i32, r:u8, value:u8) {
    for i in 0..4 {
        for j in 0..4
        {
            if tretronimo[rotate(r, i, j) as usize] != b'.' {
                board[((j as i32 + posY)*FIELD_WIDTH as i32 + (i as i32 + posX)) as usize] = value;
            }
        }
    }
}

fn get_render(board:&[u8], tretronimo:&[u8;16], posX:i32, posY:i32, r:u8, value:u8, render_tret:bool) -> String{
    let mut line:String = String::new();
    let mut vector:Vec<u8> = Vec::new();

    for i in 0..4 {
        for j in 0..4
        {
            if tretronimo[rotate(r, i, j) as usize] != b'.' {
                vector.push(((j as i32 +posY)*FIELD_WIDTH as i32 + (i as i32 + posX)) as u8);
            }
        }
    }
    for i in 0..board.len() {
        if i % FIELD_WIDTH == 0 {
            line.push('\n')
        }

        if vector.contains(&(i as u8)) && render_tret {
            line.push(int_char_mapper(value));
        }
        else {
            line.push(int_char_mapper(board[i]));
        }
    }
    return line;
}

fn int_char_mapper(x:u8) -> char {
    match x {
        0 => ' ',
        1 => 'A',
        2 => 'B',
        3 => 'C',
        4 => 'D',
        5 => 'E',
        6 => 'F',
        7 => 'G',
        8 => '=',
        9 => '#',
        _ => panic!()
    }
}

fn does_piece_fit(tetronimo:&[u8;16], rotation:u8, x:i32, y:i32, board:&[u8]) -> bool {
    for i in 0..4 {
        for j in 0..4
        {
            let pi = rotate(rotation, i, j);
            let fi = (y + j  as i32) * FIELD_WIDTH as i32 + (i as i32 + x);

            if x + i as i32 >= 0 && ((x + i as i32) < FIELD_WIDTH as i32) {
                if y + j as i32 >= 0 && ((y + j as i32) < FIELD_HIEGHT as i32) {
                    if tetronimo[pi as usize] == b'X' && board[fi as usize] != 0 {
                        return false;
                    }
                }
            }
        }
    }

    return true;
}

fn rotate(rotation:u8, x:u8, y:u8) -> u8 {
    match rotation {
        0 => 12 + y - (x * 4),
        1 => 15 - (y * 4) - x,
        2 => 3 - y + (x * 4),
        3 => y * 4 + x,
        _ => panic!()
    }
}