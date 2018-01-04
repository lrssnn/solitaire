use super::game;
use game::{Game, Card};
use game::{make_move, draw};


pub struct Player {
    pub game: Game,
    pub found_level: u8, // Maximum safe level to play to foundation
    played_this_round: bool, // Tracks if we have made any moves since the last reset of the side_deck
    restrained: bool, // Determines if we stick to the limit imposed by found_level
}

pub fn create_player(game: Game) -> Player {
    Player {
        game: game,
        found_level: 2,
        played_this_round: false,
        restrained: true,
    }
}

pub fn player_reset(player: &mut Player) {
    player.found_level = 2;
    player.played_this_round = false;
    player.restrained = true;
}

pub fn play_one_move(player: &mut Player) -> bool {

    // We want to play the first move we find in a certain heirarchy of possible move types.
    if play_hand_found(player) { return true };
    if play_pile_found(player) { return true };
    if play_pile_pile(player)  { return true };
    if play_hand_pile(player)  { return true };
    if play_draw_hand(player)  { return true };

    return false;
 
}

fn play_hand_found(player: &mut Player) -> bool {
    let mut game = &mut player.game;

    if game.hand.len() == 0 {
        return false;
    }

    let top_card = game.hand.last().unwrap().clone();

    if player.restrained && top_card.number > player.found_level {
        return false;
    }

    let target_suit = match top_card.suit {
        'H' => 0,
        'S' => 1,
        'D' => 2,
        'C' => 3,
        _ => 0,
    };

    match game.foundations[target_suit].last() {
        None =>    if top_card.number != 1 { return false; }
        Some(c) => { if c.number + 1 != top_card.number { return false; }},
    };

    // If we have reached here, we should be able to make the move
    if !make_move(&mut game, 11, 1, target_suit + 7) {
        panic!("ERROR in play_hand_found");
    };
    player.played_this_round = true;

    // Update found_level if necessary
    let mut update = true;
    for found in &game.foundations {
        match found.last() {
            None => update = false,
            Some(c) => if c.number + 1 < player.found_level { update = false },
        }
    }
    if update {
        player.found_level += 1;
    }
    return true;
}

fn play_pile_found(player: &mut Player) -> bool {
    let mut game = &mut player.game;

    for (i, pile) in game.piles.clone().iter().enumerate() {
        
        if pile.len() == 0 {
            continue;
        }

        let lowest_card = pile.last().unwrap().clone();
        
        if player.restrained && lowest_card.number > player.found_level {
            continue;
        }

        let target_suit = match lowest_card.suit {
            'H' => 0,
            'S' => 1,
            'D' => 2,
            'C' => 3,
            _ => 0,
        };

        match game.foundations[target_suit].last() {
            None =>    if lowest_card.number != 1 { continue; },
            Some(c) => if c.number + 1 != lowest_card.number { continue; },
        }

        // We should be able to make a move now
        if !make_move(&mut game, i, 1, target_suit + 7) {
            panic!("ERROR in play_pile_found");
        }
        player.played_this_round = true;

        // Update found_level if necessary
        let mut update = true;
        for found in &game.foundations {
            match found.last() {
                None => update = false,
                Some(c) => if c.number + 1 < player.found_level { update = false },
            }
        }
        if update {
            player.found_level += 1;
        }
        return true;
    }
    return false; 
}

fn play_pile_pile(player: &mut Player) -> bool {
    let mut game = &mut player.game;

    for (i, pile) in game.piles.clone().iter().enumerate() {

        if pile.len() == 0 {
            continue;
        }

        let (highest_card, depth) = get_highest_card(&pile);

        // Look at each pile other than this one for a spot to put it
        for (j, target) in game.piles.clone().iter().enumerate() {

            let lowest_card = target.last();

            if target.len() == 0 {
                // Moving to empty pile, we dont want non kings, or kings that are already on the
                // base
                if highest_card.number != 13 || depth == pile.len() {
                    continue;
                }
            } else if !game::suit_alternates(&lowest_card.unwrap(), &highest_card) 
                || !game::number_match_desc(lowest_card, &highest_card) {
                continue;
            }

            // We should be able to make a move
            if !make_move(&mut game, i, depth, j) {
                panic!("ERROR in play_pile_pile");
            }
            player.played_this_round = true;
            return true;
        }
    }
    return false;

}

fn play_hand_pile(player: &mut Player) -> bool {
    let mut game = &mut player.game;

    if game.hand.len() == 0 {
        return false;
    }

    let card = game.hand.last().unwrap().clone();

    for (j, target) in game.piles.clone().iter().enumerate() {

        let lowest_card = target.last();

        if target.len() == 0 {
            if card.number != 13 {
                continue;
            }
        } else if !game::suit_alternates(&lowest_card.unwrap(), &card)
            || !game::number_match_desc(lowest_card, &card) {
                continue;
        }

        // We should be able to make a move
        if !make_move(&mut game, 11, 1, j) {
            panic!("ERROR in play_hand_pile");
        }
        player.played_this_round = true;
        return true;
    }
    return false;
}

fn play_draw_hand(player: &mut Player) -> bool {
    let reset = draw(&mut player.game);

    if reset {
        if player.restrained {
            player.played_this_round = false;
            player.restrained = false;
            return true;
        } else if !player.played_this_round {
            return false;
        } else {
            player.played_this_round = false;
            player.restrained = true;
            return true;
        }
    } else {
        return true;
    }
}

fn get_highest_card(pile: &Vec<Card>) -> (Card, usize) {
    let mut pile = pile.clone();

    let mut next = pile.pop();
    let mut card = game::card(&'D', 0); // Dummy card which will not be used
    let mut depth = 0;

    while next.is_some() && next.clone().unwrap().up {
        card = next.unwrap();
        next = pile.pop();
        depth += 1;
    }

    return (card, depth);
}
    
