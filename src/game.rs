use std::fmt::Display;

pub struct Game {
    piles: [Vec<Card>; 7],
    side_deck: Vec<Card>,
    foundations: [Vec<Card>; 4],
}

pub struct Card {
    suit: String,
    number: u8,
}

pub fn deck() -> Vec<Card> {
    let mut deck = vec![];
    for suit in &["Hearts", "Diamonds", "Clubs", "Spades"] {
        for i in 1..14 {
            deck.push(card(suit.clone(), i));
        }
    }
    deck
}

pub fn card(suit: &str, number: u8) -> Card {
    Card { suit: suit.to_string(), number: number}
}

pub fn card_clone(card: &Card) -> Card {
    Card { suit: card.suit.clone(), number: card.number}
}

pub fn print_card(card: &Card) {
    if card.number == 1 {
        println!("Ace of {}", card.suit);
    } else if card.number == 11 {
        println!("Jack of {}", card.suit);
    } else if card.number == 12 {
        println!("Queen of {}", card.suit);
    } else if card.number == 13 {
        println!("King of {}", card.suit);
    } else {
        println!("{} of {}", card.number, card.suit);
    }
}

pub fn print_card_short(card: &Card) {
    let mut suit = card.suit.clone();
    suit.truncate(1);
    if card.number == 1 {
        println!("A{}", suit);
    } else if card.number == 11 {
        println!("J{}", suit);
    } else if card.number == 12 {
        println!("Q{}", suit);
    } else if card.number == 13 {
        println!("K{}", suit);
    } else {
        println!("{}{}", card.number, suit);
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
