extern crate ncurses;
mod game;
mod player;

use std::time;
use std::thread::sleep;

use game::*;
use std::char;

use player::Player;
use ncurses as term;

static COLOR_RED: i16 = 16;
static COLOR_BLK: i16 = 15;
static COLOR_BG: i16 = 17;
static PAIR_RED: i16 = 1;
static PAIR_BLK: i16 = 2;

static delay_ms: u64 = 0;
static test_secs: u64 = 60 * 15;


fn main() {

    let silent = profile_silent();
    let silent_end = time::Instant::now();
    let game = profile_stats_game();
    let game_end = time::Instant::now();
    let mv = profile_stats_move();
    let mv_end = time::Instant::now();
    let full = profile_full_print();
    let full_end = time::Instant::now();

    print_stats_curses(&silent.game, silent_end);
    print_stats_curses(&game.game, game_end);
    print_stats_curses(&mv.game, mv_end);
    print_stats_curses(&full.game, full_end);
    term::refresh();
    term::getch();
    
}

fn profile_silent() -> Player {
    let deck = deck();
    let mut deck = shuffle(&deck);
    let mut game = game_init();

    deal(&mut game, &mut deck);

    let mut player = player::create_player(game);

    while player.game.started.elapsed() <=  std::time::Duration::from_secs(test_secs) {
        if game_won(&mut player.game) {
            game_restart(&mut player.game);
            player::player_reset(&mut player);
        } else {
            if !player::play_one_move(&mut player) {
                game_restart(&mut player.game);
                player::player_reset(&mut player);
            }
        }
    }
    print_stats(&player.game);
    println!("");
    player
}

fn profile_stats_game() -> Player {
    let deck = deck();
    let mut deck = shuffle(&deck);
    let mut game = game_init();

    deal(&mut game, &mut deck);

    let mut player = player::create_player(game);

    while player.game.started.elapsed() < std::time::Duration::from_secs(test_secs) {
        if game_won(&mut player.game) {
            game_restart(&mut player.game);
            player::player_reset(&mut player);
        } else {
            if !player::play_one_move(&mut player) {
                game_restart(&mut player.game);
                player::player_reset(&mut player);
                print_stats(&player.game);
            }
        }
    }
    println!("");
    player
}

fn profile_stats_move() -> Player {
    let deck = deck();
    let mut deck = shuffle(&deck);
    let mut game = game_init();

    deal(&mut game, &mut deck);

    let mut player = player::create_player(game);

    while player.game.started.elapsed() < std::time::Duration::from_secs(test_secs) {
        if game_won(&mut player.game) {
            game_restart(&mut player.game);
            player::player_reset(&mut player);
        } else {
            if !player::play_one_move(&mut player) {
                game_restart(&mut player.game);
                player::player_reset(&mut player);
            }
        }
        print_stats(&player.game);
    }
    println!("");
    player
}

fn profile_full_print() -> Player{
    term::initscr();
    term::start_color();
    term::init_color(COLOR_RED, 219*4, 51*4, 47*4);
    term::init_color(COLOR_BLK, 256*4, 256*4, 256*4);
    term::init_color(COLOR_BG, 0, 0, 0);
    term::init_pair(PAIR_RED, COLOR_RED, COLOR_BG);
    term::init_pair(PAIR_BLK, COLOR_BLK, COLOR_BG);

    let deck = deck();
    let mut deck = shuffle(&deck);
    let mut game = game_init();

    deal(&mut game, &mut deck);

    let mut player = player::create_player(game);

    while player.game.started.elapsed() < std::time::Duration::from_secs(test_secs) {
        if game_won(&mut player.game) {
            game_restart(&mut player.game);
            player::player_reset(&mut player);
        } else {
            if !player::play_one_move(&mut player) {
                game_restart(&mut player.game);
                player::player_reset(&mut player);
            }
        }
        print_game(&player.game);
    }
    player
}

#[allow(dead_code)]
fn play_human() {

    let deck = deck();
    let mut deck = shuffle(&deck);
    let mut game = game_init();

    deal(&mut game, &mut deck);

    let mut ch;
    let mut src_pile;
    let mut src_depth;
    let mut dest_pile;
    loop {
        term::clear();
        if game_won(&mut game) {
            term::printw("=======================================\nWIN\n=======================================");
            game_restart(&mut game);
        }
        print_game(&game);
        term::refresh();
        let mut valid = false;
        while !valid {
            valid = false;
            loop {
                term::printw("\nSrc pile?: ");
                term::refresh();
                ch = term::getch();
                src_pile = src_index_from_char(ch);

                if src_pile != 99 { break };
            }

            if src_pile == 12 {
                draw(&mut game);
                valid = true;
            } else if src_pile == 13 {
                game_restart(&mut game);
            } else {

                loop {
                    term::printw("\nsrc depth?: ");
                    term::refresh();
                    ch = term::getch();
                    src_depth = depth_from_char(ch);
                    if src_depth != 99 { break };
                }

                loop {
                    term::printw("\ndest_pile?: ");
                    term::refresh();
                    ch = term::getch();
                    dest_pile = dest_index_from_char(ch);
                    if dest_pile != 99 { break };
                }

                term::printw(&format!("\n{}, {}, {}\n", src_pile, src_depth, dest_pile));
                term::refresh();
                valid = make_move(&mut game, src_pile, src_depth, dest_pile);
            }
        }
    }
}

// Takes a character from getch() and returns the pile index
// zero through 6 refer to the piles, h s d c refers to the foundations indices 7 through 10, 
// q draws from the hand and w draws from the side deck to the hand
// r restarts the game
fn src_index_from_char(ch: i32) -> usize {
    match char::from_u32(ch as u32).expect("Invalid char") {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        'q' => 11,
        'w' => 12,
        'r' => 13,
        _ => 99,
    }
}

fn dest_index_from_char(ch: i32) -> usize {
    match char::from_u32(ch as u32).expect("Invalid char") {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        'h' => 7,
        's' => 8,
        'd' => 9,
        'c' => 10,
        _ => 99,
    }
}

fn depth_from_char(ch: i32) -> usize {
    match char::from_u32(ch as u32).expect("Invalid char") {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ =>  99,
    }
}

fn delay() {
    wait_millis(delay_ms);
}

fn wait_millis(millis: u64) {
    let dur = time::Duration::from_millis(millis);
    sleep(dur);
}

