mod game;

use game::*;

fn main() {
    let deck = deck();
    for card in &deck {
        print_card_short(card);
    }

    let deck = shuffle(&deck);
    for card in &deck {
        print_card_short(card);
    }
}
