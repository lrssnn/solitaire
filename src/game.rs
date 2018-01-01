use std::fmt::Display;
extern crate rand;

pub struct Game {
    piles: [Vec<Card>; 7],
    side_deck: Vec<Card>,
    foundations: [Vec<Card>; 4],
}

pub struct Card {
    suit: Suit,
    number: u8,
}

pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades
}
    

pub fn deck() -> Vec<Card> {
    let mut deck = vec![];
        for i in 1..14 {
            deck.push(card(Suit::Hearts, i));
            deck.push(card(Suit::Diamonds, i));
            deck.push(card(Suit::Clubs, i));
            deck.push(card(Suit::Spades, i));
        }
    deck
}

pub fn card(suit: Suit, number: u8) -> Card {
    Card { suit: suit, number: number}
}

pub fn print_card(card: Card) {
    if card.number == 1 {
        println!("Ace of {}", suit_string(card.suit));
    } else if card.number == 11 {
        println!("Jack of {}", suit_string(card.suit));
    } else if card.number == 12 {
        println!("Queen of {}", suit_string(card.suit));
    } else if card.number == 13 {
        println!("King of {}", suit_string(card.suit));
    } else {
        println!("{} of {}", card.number, suit_string(card.suit));
    }
}

pub fn print_card_short(card: Card) {
    if card.number == 1 {
        println!("A{}", suit_string(card.suit));
    } else if card.number == 11 {
        println!("J{}", suit_string(card.suit));
    } else if card.number == 12 {
        println!("Q{}", suit_string(card.suit));
    } else if card.number == 13 {
        println!("K{}", suit_string(card.suit));
    } else {
        println!("{}{}", card.number, suit_string(card.suit));
    }
}


pub fn suit_string(suit: Suit) -> String {
    use self::Suit::*;
    match suit {
       Hearts => "Hearts".to_string(),
       Diamonds => "Diamonds".to_string(),
       Clubs => "Clubs".to_string(),
       Spades => "Spades".to_string(),
    }
}

pub fn shuffle(deck: &Vec<Card>) -> Vec<Card> {
    let mut res = vec![];
    let mut used = [false; 52];

    for card in 0..52 {
        let mut choice = rand::generate();
        while used[choice] {
            choice = rand::generate();
        }
        res.push(deck[choice]);
        used[choice] = true;
    }

    res
}
