mod game;

use game::*;
use std::io;

fn main() {
    let deck = deck();
    let mut deck = shuffle(&deck);

    let mut game = game_init();

    deal(&mut game, &mut deck);

    println!("{:?}", game);
    let mut input = String::new();
    loop {
        print_game(&game);
        println!("Src pile?: ");
        io::stdin().read_line(&mut input);
        let src_pile: usize = input.trim().parse().unwrap();
        input = String::new();

        if src_pile > 11 {
            draw(&mut game);
        } else {

            println!("src depth?: ");
            io::stdin().read_line(&mut input);
            let src_depth: usize = input.trim().parse().unwrap();
            input = String::new();

            println!("dest_pile?: ");
            io::stdin().read_line(&mut input);
            let dest_pile: usize = input.trim().parse().unwrap();
            input = String::new();

            make_move(&mut game, src_pile, src_depth, dest_pile);
        }
    }


}
