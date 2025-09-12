mod utils;
use utils::{Board, SIZE, clear_screen, pause, start_table, print_table};
use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::time::Duration;
use rand::Rng;

fn check_victory(board: &Board, player: char) -> bool {
    for i in 0..SIZE {
        if (0..SIZE).all(|j| board[i][j] == player) {
            return true;
        }
        if (0..SIZE).all(|j| board[j][i] == player) {
            return true;
        }
    }
    (0..SIZE).all(|i| board[i][i] == player)
        || (0..SIZE).all(|i| board[i][SIZE - 1 - i] == player)
}

fn check_draw(board: &Board) -> bool {
    board.iter().all(|row| row.iter().all(|&c| c != ' '))
}

// ---------------- MINIMAX ----------------
fn minimax(board: &mut Board, depth: i32, is_max: bool, p1: char, p2: char) -> i32 {
    if check_victory(board, p2) {
        return 10 - depth;
    }
    if check_victory(board, p1) {
        return depth - 10;
    }
    if check_draw(board) {
        return 0;
    }

    if is_max {
        let mut best = i32::MIN;
        for i in 0..SIZE {
            for j in 0..SIZE {
                if board[i][j] == ' ' {
                    board[i][j] = p2;
                    best = best.max(minimax(board, depth + 1, false, p1, p2));
                    board[i][j] = ' ';
                }
            }
        }
        best
    } else {
        let mut best = i32::MAX;
        for i in 0..SIZE {
            for j in 0..SIZE {
                if board[i][j] == ' ' {
                    board[i][j] = p1;
                    best = best.min(minimax(board, depth + 1, true, p1, p2));
                    board[i][j] = ' ';
                }
            }
        }
        best
    }
}

// ---------------- IA COM DIFICULDADE ----------------
fn computer_move(board: &mut Board, difficulty: u8, p1: char, p2: char) {
    let mut rng = rand::rng();

    match difficulty {
        1 => { // Fácil → jogada aleatória
            let mut empty_positions = Vec::new();
            for i in 0..SIZE {
                for j in 0..SIZE {
                    if board[i][j] == ' ' {
                        empty_positions.push((i, j));
                    }
                }
            }
            if !empty_positions.is_empty() {
                let idx = rng.random_range(0..empty_positions.len());
                let (r, c) = empty_positions[idx];
                board[r][c] = p2;
            }
        }
        2 => { // Médio → 50% aleatório, 50% minimax
            if rng.random_bool(0.5) {
                computer_move(board, 1, p1, p2);
            } else {
                computer_move(board, 3, p1, p2);
            }
        }
        _ => { // Difícil → sempre minimax
            let mut best_val = i32::MIN;
            let mut best_move = (0, 0);

            for i in 0..SIZE {
                for j in 0..SIZE {
                    if board[i][j] == ' ' {
                        board[i][j] = p2;
                        let move_val = minimax(board, 0, false, p1, p2);
                        board[i][j] = ' ';
                        if move_val > best_val {
                            best_val = move_val;
                            best_move = (i, j);
                        }
                    }
                }
            }
            let (r, c) = best_move;
            board[r][c] = p2;
        }
    }
}
// ---------------- FIM IA ----------------

fn read_char_prompt(prompt: &str) -> Option<char> {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut s = String::new();
    if io::stdin().read_line(&mut s).is_ok() {
        s.trim().chars().next()
    } else {
        None
    }
}

fn read_usize_in_range(prompt: &str, min: usize, max: usize) -> usize {
    loop {
        print!("{}", prompt);
        let _ = io::stdout().flush();
        let mut s = String::new();
        if io::stdin().read_line(&mut s).is_ok() {
            if let Ok(n) = s.trim().parse::<isize>() {
                if n >= min as isize && n <= max as isize {
                    return n as usize;
                }
            }
        }
        println!("Entrada inválida. Tente novamente.");
    }
}

fn tic_tac_toe(ai_enabled: bool, difficulty: u8, p1: char, p2: char) -> bool {
    let mut board: Board = [[' '; SIZE]; SIZE];
    start_table(&mut board);
    let mut current_player: char = p1;

    loop {
        print_table(&board);

        if current_player == p1 || !ai_enabled {
            let line = read_usize_in_range(
                &format!("Jogador {} escolha uma linha (0-{}): ", current_player, SIZE - 1),
                0,
                SIZE - 1,
            );
            let col = loop {
                let c = read_usize_in_range(
                    &format!("\nJogador {} escolha uma coluna (0-{}): ", current_player, SIZE - 1),
                    0,
                    SIZE - 1,
                );
                if board[line][c] == ' ' {
                    break c;
                } else {
                    println!("Casa ocupada, escolha outra.");
                }
            };
            board[line][col] = current_player;
        } else {
            println!("IA pensando...");
            for _ in 0..3 {
                print!(".");
                let _ = io::stdout().flush();
                thread::sleep(Duration::from_millis(800));
            }
            println!();
            computer_move(&mut board, difficulty, p1, p2);
        }

        if check_victory(&board, current_player) {
            print_table(&board);
            if ai_enabled && current_player == p2 {
                println!("\n🤖 IA ganhou!!!");
                pause();
                return true; // IA venceu
            } else {
                println!("\n🎉 Jogador {} ganhou!!!", current_player);
                pause();
                return false;
            }
        } else if check_draw(&board) {
            print_table(&board);
            println!("\n😅 Empate!");
            pause();
            return false;
        } else {
            current_player = if current_player == p1 { p2 } else { p1 };
            clear_screen();
        }
    }
}

fn main() {
    if let Some(bg) = read_char_prompt("Escolha uma cor de fundo\nBranco[1]\nCinza[2]\nPreto[3]\n:§: ") {
        if cfg!(target_os = "windows") {
            match bg {
                '1' => { let _ = Command::new("cmd").args(&["/C", "color f0"]).status(); }
                '2' => { let _ = Command::new("cmd").args(&["/C", "color 70"]).status(); }
                '3' => { let _ = Command::new("cmd").args(&["/C", "color 07"]).status(); }
                _   => { let _ = Command::new("cmd").args(&["/C", "color 40"]).status(); }
            }
        }
    }

    let ai_choice: bool = loop {
        if let Some(c) = read_char_prompt("Deseja jogar contra a IA? (1 para sim, 0 para não)\n:§: ") {
            if c == '1' { break true; }
            if c == '0' { break false; }
        }
        println!("Entrada inválida. Digite 1 ou 0.");
    };

    let mut difficulty: u8 = 3;
    if ai_choice {
        difficulty = loop {
            if let Some(c) = read_char_prompt("\nEscolha a dificuldade da IA\n1 = Fácil\n2 = Médio\n3 = Difícil\n:§: ") {
                thread::sleep(Duration::from_secs(1));
                if let Some(d) = c.to_digit(10) {
                    let d = d as u8;
                    if (1..=3).contains(&d) {
                        break d;
                    }
                }
            }
            println!("Entrada inválida. Digite 1, 2 ou 3.");
        };
    }

    // Símbolos dos jogadores
    let mut player1_symbol: char = 'x';
    let mut player2_symbol: char = 'o';

    if ai_choice {
        if let Some(s1) = read_char_prompt("\nJogador, escolha seu símbolo (X, O, ∆, ♥ ...): ") {
            player1_symbol = s1; 
        }

        loop {
            if let Some(s2) = read_char_prompt("\nAgora escolha o símbolo da IA: ") {
                if s2 != player1_symbol {
                    player2_symbol = s2;
                    break;
                } else {
                    println!("⚠️ O símbolo da IA não pode ser igual ao seu! Escolha outro.");
                }
            }
        }
    } else {
        if let Some(s1) = read_char_prompt("\nJogador 1, escolha seu símbolo (X, O, ∆, ♥ ...): ") {
            player1_symbol = s1;
        }

        loop {
            if let Some(s2) = read_char_prompt("\nJogador 2, escolha seu símbolo: ") {
                if s2 != player1_symbol {
                    player2_symbol = s2;
                    break;
                } else {
                    println!("⚠️ O símbolo do Jogador 2 não pode ser igual ao do Jogador 1! Escolha outro.");
                }
            }
        }
    }

    clear_screen();
    println!("\nIniciando...\x07\n");
    thread::sleep(Duration::from_secs(1));

    let mut ia_wins = 0;

    loop {
        clear_screen();
        println!("\n===== Jogo da Velha =====\n");

        let ia_won = tic_tac_toe(ai_choice, difficulty, player1_symbol, player2_symbol);

        if ai_choice && ia_won {
            ia_wins += 1;
            if ia_wins >= 3 {
                println!("🤖 A IA está imbatível! Vai desistir? kkk");
            }
        }

        if let Some(ans) = read_char_prompt("\nDeseja jogar novamente? (s/n): ") {
            if ans == 's' || ans == 'S' { continue; } else { break; }
        } else {
            break;
        }
    }

    println!("\n\x07==== Obrigado por Jogar ====\x07\r");
    clear_screen();
}





