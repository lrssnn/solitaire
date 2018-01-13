class Player {
    Game    game;
    int     found_level; // Maximum safe level to play to foundation
    boolean played_this_round; // Tracks if we have made any moves since the last reset of the side_deck
    boolean restrained; // Determines if we stick to the limit imposed by found_level

    public Player(Game game) {
        this.game = game;
        this.found_level = 2;
        this.played_this_round = false;
        this.restrained = true;
    }

    public void reset() {
        this.found_level = 2;
        this.played_this_round = false;
        this.restrained = true;
    }

    public boolean play_one_move() {

        // We want to play the first move we find in a certain heirarchy of possible move types.
        if (this.play_hand_found())   { return true };
        if (this.play_pile_found())   { return true };
        if (this.play_pile_pile())    { return true };
        if (this.play_hand_pile())    { return true };
        if (this.play_reveal_found()) { return true };
        if (this.play_draw_hand())    { return true };

        return false
    }

    public boolean play_hand_found() {
        Game game = this.game;

        if (game.hand.isEmpty()) {
            return false;
        }

        Card top_card = game.hand.last().clone();

        if (this.restrained && top_card.number > this.found_level) {
            return false;
        }

        char target_suit = get_target_suit(top_card.suit);

        match game.foundations[target_suit].last() {
        if (game.foundations[target_suit].isEmpty()) {
            if (top_card.number != 1) { return false; }
        } else {
            if (game.foundations[target_suit].last().number + 1 != top_card.number) { return false; },
        }

        // If we have reached here, we should be able to make the move
        if (!game.make_move(11, 1, target_suit + 7)) {
            System.out.println!("ERROR in play_hand_found");
            return false;
        };
        this.played_this_round = true;

        // Update found_level if necessary
        boolean update = true;
        for (found : game.foundations) {
            if (found.isEmpty()) {
                update = false;
            } else if (found.last().number + 1 < this.found_level) {
                update = false;
            }
        }
        if update {
            this.found_level += 1;
        }
        return true
    }

    public boolean play_pile_found() {
        Game game = player.game;

        for (i, pile) in game.piles.clone().iter().enumerate() {
        for (int i = 0; i < game.piles.length; i++) {
            Vector<Card> pile = game.piles[i];
            
            if pile.isEmpty() {
                continue;
            }

            Card lowest_card = pile.lastElement();
            
            if (this.restrained && lowest_card.number > this.found_level) {
                continue;
            }

            char target_suit = get_target_suit(lowest_card.suit);

            if (game.foundations[target_suit].isEmpty()) {
                if (lowest_card.number != 1) {
                    continue;
                } 
            } else if (game.foundations[target_suit].lastElement().number + 1 != lowest_card.number) {
                continue;
            }

            // We should be able to make a move now
            if (!make_move(&mut game, i, 1, target_suit + 7)) {
                System.out.println("ERROR in play_pile_found");
                return false;
            }
            this.played_this_round = true;

            // Update found_level if necessary
            boolean update = true;
            for (found : game.foundations) {
                if (found.isEmpty()) {
                    update = false;
                } else {
                    if (found.lastElement().number + 1 < this.found_level) {
                        update = false;
                    }
                }
            }
            if (update) {
                player.found_level += 1;
            }
            return true;
        }
        return false 
    }

    public boolean play_pile_pile() {
        Game game = this.game;

        for (int i = 0; i < game.piles.length; i++) {
        for (i, pile) in game.piles.clone().iter().enumerate() {
            Vector<Card> pile = game.piles[i];

            if (pile.isEmpty()) {
                continue;
            }

            let (highest_card, depth) = get_highest_card(pile);

            // Look at each pile other than this one for a spot to put it
            for (j, target) in game.piles.clone().iter().enumerate() {

                let lowest_card = target.last();

                if target.is_empty() {
                    // Moving to empty pile, we dont want non kings, or kings that are already on the
                    // base
                    if highest_card.number != 13 || depth == pile.len() {
                        continue;
                    }
                } else if !game::suit_alternates(lowest_card.unwrap(), &highest_card) 
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
        false

    }

    fn play_hand_pile(player: &mut Player) -> bool {
        let mut game = &mut player.game;

        if game.hand.is_empty() {
            return false;
        }

        let card = game.hand.last().unwrap().clone();

        for (j, target) in game.piles.clone().iter().enumerate() {

            let lowest_card = target.last();

            if target.is_empty() {
                if card.number != 13 {
                    continue;
                }
            } else if !game::suit_alternates(lowest_card.unwrap(), &card)
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
        false
    }

    fn play_draw_hand(player: &mut Player) -> bool {
        let reset = draw(&mut player.game);

        if reset {
            if player.restrained {
                player.played_this_round = false;
                player.restrained = false;
                true
            } else if !player.played_this_round {
                false
            } else {
                player.played_this_round = false;
                player.restrained = true;
                true
            }
        } else {
            true
        }
    }

    fn play_reveal_found(player: &mut Player) -> bool {
        let mut game = &mut player.game;

        // We are looking for foundation cards hidden in stacks that we can reveal.
        // Find which card each foundation needs next, if that card is present in a stack, look for a
        // place to move the cards hiding it
        for (suit, found) in game.foundations.clone().iter().enumerate() {
            if found.is_empty() { continue; }
            let target = target_card(suit, found);
            
            if player.restrained && target.number > player.found_level {
                continue;
            }

            for (i, pile) in game.piles.clone().iter().enumerate() {
                for (depth, card) in pile.iter().enumerate() {
                    let depth = pile.len() - depth - 1;
                    if card_match_exact(&card, &target) && depth != 0 {
                        for (j, dest_pile) in game.piles.clone().iter().enumerate() {
                            if !dest_pile.is_empty() {
                                let bottom_card = dest_pile.last();
                                if bottom_card.is_none() {
                                    continue;
                                }
                                let bottom_card = bottom_card.unwrap();
                                if card_match_functional(&bottom_card, &card) {
                                    // We can make a move
                                    if !make_move(&mut game, i, depth, j){
                                        println!("Error in play_reveal_found");
                                        panic!();
                                    }
                                    player.played_this_round = true;
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn target_card(suit: usize, pile: &[Card]) -> Card {
        let mut card = pile.last().unwrap();
        let mut res = card.clone();
        res.number += 1;
        res
    }

    fn card_match_exact(a: &Card, b: &Card) -> bool {
        a.up && b.up && a.number == b.number && a.suit == b.suit
    }

    fn card_match_functional(a: &Card, b: &Card) -> bool {
        if a.number == b.number {
            match a.suit {
                'H' | 'D' => return b.suit == 'H' || b.suit == 'D',
                'C' | 'S' => return b.suit == 'C' || b.suit == 'S',
                _ => return false,
            }
        } else {
            return false;
        }
    }


    fn get_highest_card(pile: &[Card]) -> (Card, usize) {
        let mut pile = pile.to_owned();

        let mut next = pile.pop();
        let mut card = game::card(&'D', 0); // Dummy card which will not be used
        let mut depth = 0;

        while next.is_some() && next.clone().unwrap().up {
            card = next.unwrap();
            next = pile.pop();
            depth += 1;
        }

        (card, depth)
    }
        
