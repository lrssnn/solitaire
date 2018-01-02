extern crate rand;
use self::rand::Rng;
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
        match game.side_deck.first() { 
            None => "   ".to_string() , 
            Some(c) => card_string(&c)},

        match game.foundations[0].first() { 
            None => "   ".to_string() , 
            Some(c) => card_string(&c) },
        match game.foundations[1].first() { 
            None => "   ".to_string() , 
            Some(c) => card_string(&c) },
        match game.foundations[2].first() { 
            None => "   ".to_string() , 
            Some(c) => card_string(&c) },
        match game.foundations[3].first() { 
            None => "   ".to_string() , 
            Some(c) => card_string(&c) }
            );

    println!("=============================");
    let mut cards = true;
    let mut row = 0;
    while cards {
        cards = false;
        for pile in &game.piles {
            if row < pile.len() {
                cards = true;
                print!("{} ", card_str_disp(&pile[row]));
            } else {
                print!("    ");
            }
        }
        row += 1;
        println!("");
    }
}

// Make a move. pile indices start at zero, naturally, with piles 7 - 10 representing the
// foundations (Hearts, Spades, Diamonds, Clubs)
// src_depth refers to the number of cards to take from the source pile.
pub fn make_move(game: &mut Game, src_pile: usize, src_depth: usize, dest_pile: usize) -> bool {
    // Make sure that there is a card where we want to take from
    if game.piles[src_pile].len() < src_depth {
        println!("Error: Trying to take non-existant card");
        return false;
    }

    // For convenience in checking validity
    let card = (game.piles[src_pile][game.piles[src_pile].len() - src_depth]).clone();
    
    // Make sure the cards are a legal move
    if dest_pile > 6 {
        // Trying to move to a foundation:
        //   - Only one card allowed
        //   - Suit must match
        //   - Number must be one higher
        if src_depth != 1 || 
          !suit_match(dest_pile, &card) || 
          !number_match_asc(game.foundations[dest_pile - 7].len(), &card) {
            println!("Error: Failed attempt to move to foundation");
            return false;
           }

        // Validation has passed:
        let card = game.piles[src_pile].pop().unwrap();
        game.foundations[dest_pile - 7].push(card);
        // Make sure the last card in the pile is revealed if necessary
        match game.piles[src_pile].last_mut() {
            None => {},
            Some(c) => {c.up = true}
        };
        return true;
    }

    // Trying to move to another pile (we already know the source stack exists):
    //   - Suit colour must alternate
    //   - Base card number must be one lower
    if !suit_alternates(&card, game.piles[dest_pile].last().unwrap()) ||
      !number_match_desc(game.piles[dest_pile].last(), &card) {
        println!("Error: Failed attempt to move to pile");
        return false;
      }

    // Validation has passed:
    let split_index = game.piles[src_pile].len() - src_depth;
    let mut cards = game.piles[src_pile].split_off(split_index);
    game.piles[dest_pile].append(&mut cards);
    match game.piles[src_pile].last_mut() {
        None => {},
        Some(c) => {c.up = true}
    };
    return true;

}

pub fn suit_match(dest: usize, card: &Card) -> bool {
    match dest {
        7 => return card.suit == 'H',
        8 => return card.suit == 'S',
        9 => return card.suit == 'D',
        10 => return card.suit == 'C',
        _ => return false,
    }
}

pub fn suit_alternates(a: &Card, b: &Card) -> bool {
    match a.suit {
        'H' | 'D' => { return b.suit == 'C' || b.suit == 'S'},
        'C' | 'S' => { return b.suit == 'H' || b.suit == 'D'},
        _ => return false,
    };
}

pub fn number_match_asc(base: usize, card: &Card) -> bool {
    return card.number - 1 == base as u8;
}

pub fn number_match_desc(dest: Option<&Card>, new: &Card) -> bool {
    match dest {
        None => return new.number == 13,
        Some(c) => return c.number - 1 == new.number,
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
    else { " XX".to_string() }
}

pub fn card_string(card: &Card) -> String {
    if card.number == 1 {
        format!(" A{}", card.suit)
    } else if card.number == 11 {
        format!(" J{}", card.suit)
    } else if card.number == 12 {
        format!(" Q{}", card.suit)
    } else if card.number == 13 {
        format!(" K{}", card.suit)
    } else {
        format!("{:2}{}", card.number, card.suit)
    }
}

pub fn print_card_short(card: &Card) {
    if card.number == 1 {
        println!(" A{}", card.suit);
    } else if card.number == 11 {
        println!(" J{}", card.suit);
    } else if card.number == 12 {
        println!(" Q{}", card.suit);
    } else if card.number == 13 {
        println!(" K{}", card.suit);
    } else {
        println!("{:2}{}", card.number, card.suit);
    }
}

pub fn shuffle(deck: &Vec<Card>) -> Vec<Card> {
    let mut rng = rand::thread_rng();
    let mut res = vec![];
    let mut used = [false; 52];

    for card in 0..52 {
        let mut choice: usize = rng.gen::<usize>() % 52;
        while used[choice] {
            choice = rng.gen::<usize>() % 52;
        }
        res.push(card_clone(&deck[choice]));
        used[choice] = true;
    }

    res
}
