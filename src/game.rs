extern crate rand;
use self::rand::Rng;
use std::clone::Clone;
use std::time;
use std::fmt;
use std::io;
use std::io::Write;

use super::ncurses as term;

use super::PAIR_RED;
use super::PAIR_BLK;

#[derive(Debug)]
pub struct Game {
    pub piles: [Vec<Card>; 7],
    pub side_deck: Vec<Card>,
    pub hand: Vec<Card>,
    pub foundations: [Vec<Card>; 4],
    pub score: isize,
    pub games: usize,
    pub wins: usize,
    pub moves: usize,
    pub started: time::Instant,
}

#[derive(Debug, Clone)]
pub struct Card {
    pub suit: char,
    pub number: u8,
    pub up: bool,
}

pub fn game_init() -> Game {
    Game {
        piles: [vec![],vec![],vec![],vec![], vec![], vec![], vec![]],
        side_deck: vec![],
        hand: vec![],
        foundations: [vec![], vec![], vec![], vec![]],
        score: 0,
        games: 0,
        wins: 0,
        moves: 0,
        started: time::Instant::now(),
    }
}

// Creates a new shuffled deck and begins the game
pub fn game_restart(game: &mut Game) {
    let deck = deck();
    let mut deck = shuffle(&deck);
    deal(game, &mut deck);
    game.score -= 52;
    game.games += 1;
}

// Deals the given deck into the given game. The deck given should ALREADY BE SHUFFLED
pub fn deal(game: &mut Game, deck: &mut Vec<Card>) {
    game.piles = [vec![],vec![],vec![],vec![], vec![], vec![], vec![]];
    game.foundations = [vec![], vec![], vec![], vec![]];
    game.hand = vec![];

    // Populate the piles
    for (i, pile) in game.piles.iter_mut().enumerate() {
        for _j in 0..i {
            pile.push(deck.pop().unwrap());
        }
        pile.push(reveal(&deck.pop().unwrap()));
    }

    game.side_deck = deck.to_vec();
}

pub fn print_stats(game: &Game) {
    let elapsed = Dur { dur: game.started.elapsed() };
    print!("{} | {} g/s, {} w/s ",
                          elapsed,
                          game.games / (elapsed.dur.as_secs() as usize + 1),
                          game.wins  / (elapsed.dur.as_secs() as usize + 1));
    print!("Score: {} | Winrate: {:4.2}% ({}/{}) | Moves: {}       \r", 
                          game.score, 
                          100.0 *(game.wins as f32 / game.games as f32),
                          game.wins,
                          game.games,
                          game.moves);
    io::stdout().flush();

}

pub fn print_stats_curses(game: &Game, ended: time::Instant) {
    let elapsed = Dur { dur: ended.duration_since(game.started) };
    term::printw(&format!("{} | {} g/s, {} w/s ",
                          elapsed,
                          game.games / (elapsed.dur.as_secs() as usize + 1),
                          game.wins  / (elapsed.dur.as_secs() as usize + 1)));
    term::printw(&format!("Score: {} | Winrate: {:4.2}% ({}/{}) | Moves: {}      \n\r", 
                          game.score, 
                          100.0 *(game.wins as f32 / game.games as f32),
                          game.wins,
                          game.games,
                          game.moves));
}


pub fn print_game(game: &Game) {
    term::clear();
    let elapsed = Dur { dur: game.started.elapsed() };
    term::printw(&format!("{} | {} g/s, {} w/s ",
                          elapsed,
                          game.games / (elapsed.dur.as_secs() as usize + 1),
                          game.wins  / (elapsed.dur.as_secs() as usize + 1)));
    term::printw(&format!("Score: {} | Winrate: {:4.2}% ({}/{}) | Moves: {}\n", 
                          game.score, 
                          100.0 *(game.wins as f32 / game.games as f32),
                          game.wins,
                          game.games,
                          game.moves));
    // Top line of game in parts:
    // 1. Side deck size
    term::printw(&format!("({:2})[", game.side_deck.len()));
    // 2. Hand
    match game.hand.last() {
        None => (),
        Some(c) => set_colour(c),
    }
    term::printw(&format!("{}", 
                          match game.hand.last() {
                              None => "   ".to_string(),
                              Some(c) => card_string(c),
                          }));
    match game.hand.last() {
        None => (),
        Some(c) => clear_colour(c),
    }
    term::printw("]  ");

    // 3. Foundations
    for found in &game.foundations {
        term::printw("[");
        match found.last() {
            None => (),
            Some(c) => set_colour(c),
        }
        term::printw(&format!("{}",
                              match found.last() {
                                  None => "   ".to_string(),
                                  Some(c) => card_string(c),
                              }));
        match found.last() {
            None => (),
            Some(c) => clear_colour(c),
        }
        term::printw("]");
    }
    term::printw("\n");
    term::printw("=============================\n");
    let mut cards = true;
    let mut row = 0;
    while cards {
        cards = false;
        for pile in &game.piles {
            if row < pile.len() {
                cards = true;
                set_colour(&pile[row]);
                term::printw(&format!("{} ", card_str_disp(&pile[row])));
                clear_colour(&pile[row]);
            } else {
                term::printw("    ");
            }
        }
        row += 1;
        term::printw("\n");
    }
    term::refresh();
}

pub fn game_won(game: &mut Game) -> bool {
    for found in &game.foundations {
        match found.last() {
            None => {return false},
            Some(c) => { if c.number != 13 {return false}}
        }
    }
    game.wins += 1;
    game.score += 5000;
    true
}

// Make a move. pile indices start at zero, naturally, with piles 7 - 10 representing the
// foundations (Hearts, Spades, Diamonds, Clubs)
// src_depth refers to the number of cards to take from the source pile.
pub fn make_move(game: &mut Game, src_pile: usize, src_depth: usize, dest_pile: usize) -> bool {

    game.moves += 1; // Don't talk to me
    
    // If src pile is 11, we are taking from the draw hand:
    //   - must be a card in the hand to take
    //   - depth must be one
    if src_pile == 11 {
        if game.hand.is_empty() {
            term::printw("Error: No cards in hand\n");
            return false;
        } else if src_depth != 1 {
            term::printw("Error: Cannot take more than one card from hand\n");
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
        term::printw("Error: Trying to take non-existant card\n");
        return false;
    }
    
    if dest_pile > 6 {
        move_pile_found(game, src_pile, src_depth, dest_pile)
    } else {
        move_pile_pile(game, src_pile, src_depth, dest_pile)
    }

}
pub fn move_hand_found(game: &mut Game, dest_pile: usize) -> bool {
    // Moving to a foundation:
    //   - Suit must match
    //   - number must be one higher
    let card = game.hand.last().unwrap().clone();
    if !suit_match(dest_pile, &card) {
        term::printw("Error: Suits must match on the foundations\n");
        return false;
    } else if !number_match_asc(game.foundations[dest_pile - 7].len(), &card) {
        term::printw("Error: Numbers must ascend on the foundations\n");
        return false;
    }

    let card = game.hand.pop().unwrap();
    game.foundations[dest_pile - 7].push(card);
    game.score += 5;
    true
}


pub fn move_hand_pile(game: &mut Game, dest_pile: usize) -> bool {
    // Moving to a pile:
    //   - suit must alternate
    //   - number must be one lower
    let card = game.hand.last().unwrap().clone();
    if card.number == 13 {
        // King:
        //   - Destination pile must be empty
        if !game.piles[dest_pile].is_empty() {
            term::printw("Error: Kings can only be moved to empty piles\n");
            return false;
        }
    } else if game.piles[dest_pile].is_empty() {
        term::printw("Error: Only Kings can move to empty piles\n");
        return false;
    } else if !suit_alternates(&card, game.piles[dest_pile].last().unwrap()) {
        term::printw("Error: Suits must alternate on the piles\n");
        return false;
    } else if !number_match_desc(game.piles[dest_pile].last(), &card) {
        term::printw("Error: Numbers must decrease by one on the piles\n");
        return false;
    }
    
    let card = game.hand.pop().unwrap();
    game.piles[dest_pile].push(reveal(&card));
    true
}
pub fn move_pile_found(game: &mut Game, src_pile: usize, src_depth: usize, dest_pile: usize) -> bool {
    // Trying to move to a foundation:
    //   - Only one card allowed
    //   - Suit must match
    //   - Number must be one higher
    let card = (game.piles[src_pile][game.piles[src_pile].len() - src_depth]).clone();
    if src_depth != 1 {
        term::printw("Error: Cannot move more than one card to foundation\n");
        return false;
    } else if !suit_match(dest_pile, &card) {
        term::printw("Error: Suits must match on the foundation\n");
        return false;
    } else if !number_match_asc(game.foundations[dest_pile - 7].len(), &card) {
        term::printw("Error: Numbers must ascend by one on the foundation\n");
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
    game.score += 5;
    true
}

pub fn move_pile_pile(game: &mut Game, src_pile: usize, src_depth: usize, dest_pile: usize) -> bool {

    // Trying to move to another pile (we already know the source stack exists):

    let card = (game.piles[src_pile][game.piles[src_pile].len() - src_depth]).clone();

    if card.number == 13 {
        // King:
        //   - Destination pile must be empty
        if !game.piles[dest_pile].is_empty() {
            term::printw("Error: Kings can only be moved to empty piles\n");
            return false;
        }
    } else if game.piles[dest_pile].is_empty() {
            term::printw("Error: Cannot move non-king to empty pile\n");
            return false;
    } else if !suit_alternates(&card, game.piles[dest_pile].last().unwrap()) {
        term::printw("Error: Suit colours must alternate on piles\n");
        return false;
    } else if !number_match_desc(game.piles[dest_pile].last(), &card) {
        term::printw("Error: Numbers must decrease by one on piles\n");
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
    true
}

// Returns whether the hand was reset
pub fn draw(game: &mut Game) -> bool {
    if game.side_deck.is_empty() {
        game.side_deck = game.hand.clone();
        game.side_deck.reverse();
        game.hand = vec![];
        true
    } else {
        let mut moved = 0;
        while moved < 3 && !game.side_deck.is_empty() {
            game.hand.push(reveal(&game.side_deck.pop().unwrap()));
            moved += 1;
        }
        false
    }
}    

pub fn suit_match(dest: usize, card: &Card) -> bool {
    match dest {
        7 => card.suit == 'H',
        8 => card.suit == 'S',
        9 => card.suit == 'D',
        10 => card.suit == 'C',
        _ => false,
    }
}

pub fn suit_alternates(a: &Card, b: &Card) -> bool {
    match a.suit {
        'H' | 'D' => { b.suit == 'C' || b.suit == 'S'},
        'C' | 'S' => { b.suit == 'H' || b.suit == 'D'},
        _ => false,
    }
}

pub fn number_match_asc(base: usize, card: &Card) -> bool {
    card.number - 1 == base as u8
}

pub fn number_match_desc(dest: Option<&Card>, new: &Card) -> bool {
    match dest {
        None => new.number == 13,
        Some(c) => c.number - 1 == new.number,
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
    Card { suit: *suit, number: number, up: false}
}

pub fn card_clone(card: &Card) -> Card {
    Card { suit: card.suit, number: card.number, up: card.up}
}

pub fn reveal(card: &Card) -> Card {
    Card { suit: card.suit, number: card.number, up: true}
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

pub fn shuffle(deck: &[Card]) -> Vec<Card> {
    let mut rng = rand::thread_rng();
    let mut res = vec![];
    let mut used = [false; 52];

    for _card in 0..52 {
        let mut choice: usize = rng.gen::<usize>() % 52;
        while used[choice] {
            choice = rng.gen::<usize>() % 52;
        }
        res.push(card_clone(&deck[choice]));
        used[choice] = true;
    }

    res
}

pub fn set_colour(card: &Card) {
    if card.up {
       match card.suit {
           'H' | 'D' => {
               term::attron(term::COLOR_PAIR(PAIR_RED));
           },
           'C' | 'S' => {
               term::attron(term::COLOR_PAIR(PAIR_BLK));
           },
           _ => ()
       }
    }
}

pub fn clear_colour(card: &Card) {
    if card.up {
       match card.suit {
           'H' | 'D' => {
               term::attroff(term::COLOR_PAIR(PAIR_RED));
           },
           'C' | 'S' => {
               term::attroff(term::COLOR_PAIR(PAIR_BLK));
           },
           _ => ()
       }
    }
}
    
struct Dur {
    dur: time::Duration,
}
impl fmt::Display for Dur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut secs = self.dur.as_secs();

        let hrs = secs/3600;
        secs -= hrs * 3600;

        let mins = secs/60;
        secs -= mins * 60;
        write!(f, "{:02}:{:02}:{:02}", hrs, mins, secs)
    }
}
