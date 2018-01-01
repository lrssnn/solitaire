use std::fmt::Display;
use std::clone::Clone;

#[derive(Debug)]
pub struct Game {
    piles: [Vec<Card>; 7],
    side_deck: Vec<Card>,
    foundations: [Vec<Card>; 4],
}

#[derive(Debug, Clone)]
pub struct Card {
    suit: char,
    number: u8,
    up: bool,
}

pub fn game_init() -> Game {

    Game {
        piles: [vec![],vec![],vec![],vec![], vec![], vec![], vec![]],
        side_deck: vec![],
        foundations: [vec![], vec![], vec![], vec![]],
    }

}

// Deals the given deck into the given game. The deck given should ALREADY BE SHUFFLED
pub fn deal(game: &mut Game, deck: &mut Vec<Card>) {
    game.foundations = [vec![], vec![], vec![], vec![]];

    // Populate the piles
    for (i, pile) in game.piles.iter_mut().enumerate() {
        for j in 0..i {
            println!("Pile {}, card {} (down)", i, j);
            pile.push(deck.pop().unwrap());
        }
        println!("Pile {}, card up", i);
        pile.push(reveal(&deck.pop().unwrap()));
    }

    game.side_deck = deck.to_vec();
}

pub fn print_game(game: &Game) {
    println!("[{}]    [{}][{}][{}][{}]",
        match game.side_deck.first() { None => "  ".to_string() , Some(c) => card_string(&c) },
        match game.foundations[0].first() { None => "  ".to_string() , Some(c) => card_string(&c) },
        match game.foundations[1].first() { None => "  ".to_string() , Some(c) => card_string(&c) },
        match game.foundations[2].first() { None => "  ".to_string() , Some(c) => card_string(&c) },
        match game.foundations[3].first() { None => "  ".to_string() , Some(c) => card_string(&c) });

    println!("=========================");
    let mut cards = true;
    let mut row = 0;
    while cards {
        cards = false;
        for pile in &game.piles {
            if row < pile.len() {
                cards = true;
                print!("{} ", card_str_disp(&pile[row]));
            } else {
                print!("   ");
            }
        }
        row += 1;
        println!("");
    }
}

pub fn deck() -> Vec<Card> {
    let mut deck = vec![];
    for suit in &['H', 'D', 'C', 'S'] {
        for i in 1..14 {
            deck.push(card(suit, i));
        }
    }
    deck
}

pub fn card(suit: &char, number: u8) -> Card {
    Card { suit: suit.clone(), number: number, up: false}
}

pub fn card_clone(card: &Card) -> Card {
    Card { suit: card.suit.clone(), number: card.number, up: card.up}
}

pub fn reveal(card: &Card) -> Card {
    Card { suit: card.suit.clone(), number: card.number, up: true}
}

pub fn card_str_disp(card: &Card) -> String {
    if card.up { card_string(card) }
    else { "XX".to_string() }
}

pub fn card_string(card: &Card) -> String {
    if card.number == 1 {
        format!("A{}", card.suit)
    } else if card.number == 11 {
        format!("J{}", card.suit)
    } else if card.number == 12 {
        format!("Q{}", card.suit)
    } else if card.number == 13 {
        format!("K{}", card.suit)
    } else {
        format!("{}{}", card.number, card.suit)
    }
}

pub fn print_card_short(card: &Card) {
    if card.number == 1 {
        println!("A{}", card.suit);
    } else if card.number == 11 {
        println!("J{}", card.suit);
    } else if card.number == 12 {
        println!("Q{}", card.suit);
    } else if card.number == 13 {
        println!("K{}", card.suit);
    } else {
        println!("{}{}", card.number, card.suit);
    }
}

pub fn shuffle(deck: &Vec<Card>) -> Vec<Card> {
    let mut res = vec![];
    let mut used = [false; 52];

    for card in 0..52 {
        /*
        let mut choice = rand::generate();
        while used[choice] {
            choice = rand::generate();
        }
        */
        let choice = card;
        res.push(card_clone(&deck[choice]));
        used[choice] = true;
    }

    res
}
