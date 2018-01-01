mod game;

use game::*;

fn main() {
    let deck = deck();
    let mut deck = shuffle(&deck);

    let mut game = game_init();

    deal(&mut game, &mut deck);

    println!("{:?}", game);

    print_game(&game);
}
