extern crate rand;
use self::rand::Rng;
use std::clone::Clone;

#[derive(Debug)]
pub struct Game {
    piles: [Vec<Card>; 7],
    side_deck: Vec<Card>,
    hand: Vec<Card>,
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
        hand: vec![],
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
    println!("({})[{}]  [{}][{}][{}][{}]",
        game.side_deck.len(),
        match game.hand.last() { 
            None => "   ".to_string() , 
            Some(c) => card_string(&c)},

        match game.foundations[0].last() { 
            None => "(H)".to_string() , 
            Some(c) => card_string(&c) },
        match game.foundations[1].last() { 
            None => "(S)".to_string() , 
            Some(c) => card_string(&c) },
        match game.foundations[2].last() { 
            None => "(D)".to_string() , 
            Some(c) => card_string(&c) },
        match game.foundations[3].last() { 
            None => "(C)".to_string() , 
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

    // If src pile is 11, we are taking from the draw hand:
    //   - must be a card in the hand to take
    //   - depth must be one
    if src_pile == 11 {
        if game.hand.len() == 0 {
            println!("Error: No cards in hand");
            return false;
        } else if src_depth != 1 {
            println!("Error: Cannot take more than one card from hand");
            return false;
        }

        // Determine target area
        if dest_pile > 6 {
            return move_hand_found(game, dest_pile);
        } else {
            return move_hand_pile(game, dest_pile);
        }
        
    }
        
        
    // Make sure that there is a card where we want to take from
    if game.piles[src_pile].len() < src_depth {
        println!("Error: Trying to take non-existant card");
        return false;
    }
    
    if dest_pile > 6 {
        return move_pile_found(game, src_pile, src_depth, dest_pile);
    } else {
        return move_pile_pile(game, src_pile, src_depth, dest_pile);
    }

}
pub fn move_hand_found(game: &mut Game, dest_pile: usize) -> bool {
    // Moving to a foundation:
    //   - Suit must match
    //   - number must be one higher
    let card = game.hand.last().unwrap().clone();
    if !suit_match(dest_pile, &card) {
        println!("Error: Suits must match on the foundations");
        return false;
    } else if !number_match_asc(game.foundations[dest_pile - 7].len(), &card) {
        println!("Error: Numbers must ascend on the foundations");
        return false;
    }

    let card = game.hand.pop().unwrap();
    game.foundations[dest_pile - 7].push(card);
    return true;
}


pub fn move_hand_pile(game: &mut Game, dest_pile: usize) -> bool {
    // Moving to a pile:
    //   - suit must alternate
    //   - number must be one lower
    let card = game.hand.last().unwrap().clone();
    if card.number == 13 {
        // King:
        //   - Destination pile must be empty
        if game.piles[dest_pile].len() != 0 {
            println!("Error: Kings can only be moved to empty piles");
            return false;
        }
    } else {
        if game.piles[dest_pile].len() == 0 {
            println!("Error: Only Kings can move to empty piles");
            return false;
        } else if !suit_alternates(&card, game.piles[dest_pile].last().unwrap()) {
            println!("Error: Suits must alternate on the piles");
            return false;
        } else if !number_match_desc(game.piles[dest_pile].last(), &card) {
            println!("Error: Numbers must decrease by one on the piles");
            return false;
        }
    }
    
    let card = game.hand.pop().unwrap();
    game.piles[dest_pile].push(reveal(&card));
    return true;
}
pub fn move_pile_found(game: &mut Game, src_pile: usize, src_depth: usize, dest_pile: usize) -> bool {
    // Trying to move to a foundation:
    //   - Only one card allowed
    //   - Suit must match
    //   - Number must be one higher
    let card = (game.piles[src_pile][game.piles[src_pile].len() - src_depth]).clone();
    if src_depth != 1 {
        println!("Error: Cannot move more than one card to foundation");
        return false;
    } else if !suit_match(dest_pile, &card) {
        println!("Error: Suits must match on the foundation");
        return false;
    } else if !number_match_asc(game.foundations[dest_pile - 7].len(), &card) {
        println!("Error: Numbers must ascend by one on the foundation");
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

pub fn move_pile_pile(game: &mut Game, src_pile: usize, src_depth: usize, dest_pile: usize) -> bool {

    // Trying to move to another pile (we already know the source stack exists):

    let card = (game.piles[src_pile][game.piles[src_pile].len() - src_depth]).clone();

    if card.number == 13 {
        // King:
        //   - Destination pile must be empty
        if game.piles[dest_pile].len() != 0 {
            println!("Error: Kings can only be moved to empty piles");
            return false;
        }
    } else {
        // Not a king: 
        //   - Destination cannot be empty
        //   - Suit colour must alternate
        //   - Base card number must be one lower
        if game.piles[dest_pile].len() == 0 {
            println!("Error: Cannot move non-king to empty pile");
            return false;
        } else if !suit_alternates(&card, game.piles[dest_pile].last().unwrap()) {
            println!("Error: Suit colours must alternate on piles");
            return false;
        } else if !number_match_desc(game.piles[dest_pile].last(), &card) {
            println!("Error: Numbers must decrease by one on piles");
            return false;
        }
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

pub fn draw(game: &mut Game) {
    println!("Draw: ");
    println!("Side deck: ");
    for card in &game.side_deck {
        println!("  {:?}", card);
    }
    println!("hand: ");
    for card in &game.hand {
        println!("  {:?}", card);
    }
    if game.side_deck.len() == 0 {
        game.side_deck = game.hand.clone();
        game.hand = vec![];
    } else {
        let mut moved = 0;
        while moved < 3 && game.side_deck.len() > 0 {
            game.hand.push(game.side_deck.pop().unwrap());
            moved += 1;
        }
    }
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
