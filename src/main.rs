extern crate ncurses;
mod game;

use game::*;
use std::io;
use std::char;

use ncurses as term;

fn main() {
    let deck = deck();
    let mut deck = shuffle(&deck);

    term::initscr();

    let mut game = game_init();

    deal(&mut game, &mut deck);

    let mut ch;
    loop {
            term::clear();
        if game_won(&mut game) {
            term::printw("============================================\nWIN\n============================================");
            game_restart(&mut game);
        }
        print_game(&game);
        term::printw("Src pile?: ");
        term::refresh();
        ch = term::getch();
        let src_pile = index_from_char(ch);

        if src_pile == 12 {
            draw(&mut game);
        } else if src_pile >= 13 {
            game_restart(&mut game);
        } else {

            term::printw("\nsrc depth?: ");
            term::refresh();
            ch = term::getch();
            let src_depth = index_from_char(ch);

            term::printw("\ndest_pile?: ");
            term::refresh();
            ch = term::getch();
            let dest_pile = index_from_char(ch);

            term::printw(&format!("\n{}, {}, {}\n", src_pile, src_depth, dest_pile));
            make_move(&mut game, src_pile, src_depth, dest_pile);
        }
    }


}

// Takes a character from getch() and returns the pile index
// zero through 6 refer to the piles, h s d c refers to the foundations indices 7 through 10, 
// q draws from the hand and w draws from the side deck to the hand
// r restarts the game
fn index_from_char(ch: i32) -> usize {
    match char::from_u32(ch as u32).expect("Invalid char") {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        'h' => 7,
        's' => 8,
        'd' => 9,
        'c' => 10,
        'q' => 11,
        'w' => 12,
        'r' => 13,
        _ => 99,
    }
}
