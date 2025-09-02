use std::io::{self, Write};
use std::process::Command;

pub const SIZE: usize = 3;
pub type Board = [[char; SIZE]; SIZE];

pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(&["/C", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }
}

pub fn pause() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(&["/C", "pause"]).status();
    } else {
        println!("Pressione Enter para continuar...");
        let mut s = String::new();
        let _ = io::stdin().read_line(&mut s);
    }
}

pub fn start_table(board: &mut Board) {
    for i in 0..SIZE {
        for j in 0..SIZE {
            board[i][j] = ' ';
        }
    }
}

pub fn print_table(board: &Board) {
    // Imprime os índices das colunas
    print!("\n     ");
    for y in 0..SIZE {
        print!(" ${}  ", y);
    }
    println!("\n   %====#=!|!=#====%");

    for x in 0..SIZE {
        print!(" {} |", x);
        for y in 0..SIZE {
            print!(" {} ", board[x][y]);
            if y < SIZE - 1 {
                print!("|");
            }
        }
        println!();
        if x < SIZE - 1 {
            println!("   -~-+-~-+-~-");
        }
    }

    println!("   #====%=!|!=%====#");
}
