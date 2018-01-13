import java.util.Vector;
import java.time.Instant;
import java.time.Duration;

class Game {
    public Vector<Card>[] piles;
    public Vector<Card> side_deck,
    public Vector<Card> hand,
    public Vector<Card>[] foundations,
    public int score,
    public int games,
    public int wins,
    public int moves,
    public Instant started,

    public Game() {

        piles = new Vector<Card>[7];
        for(int i = 0; i < 7; i++) piles[i] = new Vector<Card>();

        foundations = new Vector<Card>[4];
        for(int i = 0; i < 4; i++) foundations[i] = new Vector<Card>();

        side_deck = new Vector<Card>();
        hand      = new Vector<Card>();
        score     = 0;
        games     = 0;
        wins      = 0;
        moves     = 0;
        started   = Instant.now();
    }

    // Creates a new shuffled deck and begins the game
    public void restart() {
        Card[] deck = deck();
        Card[] shuffled = shuffle(&deck);
        this.deal(shuffled);
        this.score -= 52;
        this.games += 1;
    }

    // Deals the given deck into the given game. The deck given should ALREADY BE SHUFFLED
    public void deal(Card[] deck) {
        this.piles = [Vector<Card>(),Vector<Card>(),Vector<Card>(),Vector<Card>(), 
                     Vector<Card>(), Vector<Card>(), Vector<Card>()];
        this.foundations = [Vector<Card>(),Vector<Card>(),Vector<Card>(),Vector<Card>()];
        this.hand = Vector<Card>();

        // Populate the piles
        for (int i = 0; i < this.piles.length(); i++) {
            for (int j = 0; j < i; j++) {
                this.piles[i].push(deck.pop());
            }
            this.piles[i].push(reveal(deck.pop()));
        }

        game.side_deck = deck;
    }

    public void print_stats() {
        System.out.print("{} | {} g/s, {} w/s ",
                              Duration.between(this.started, Instant.now()).toString(),
                              game.games / (elapsed.dur.as_secs() as usize + 1),
                              game.wins  / (elapsed.dur.as_secs() as usize + 1));
        System.out.print("Score: {} | Winrate: {:4.2}% ({}/{}) | Moves: {}       \r", 
                              game.score, 
                              100.0 *(game.wins as f32 / game.games as f32),
                              game.wins,
                              game.games,
                              game.moves);
        System.out.flush();

    }

/*
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
*/

    public boolean is_won() {
        for (Vector<Card> found : this.foundations) {
            if (found.size() != 13) {
                return false;
            }
        }
        this.wins += 1;
        this.score += 5000;
        return true;
    }

    // Make a move. pile indices start at zero, naturally, with piles 7 - 10 representing the
    // foundations (Hearts, Spades, Diamonds, Clubs)
    // src_depth refers to the number of cards to take from the source pile.
    public boolean make_move(int src_pile, int src_depth, int dest_pile) {

        this.moves += 1; // Don't talk to me
        
        // If src pile is 11, we are taking from the draw hand:
        //   - must be a card in the hand to take
        //   - depth must be one
        if (src_pile == 11) {
            if (this.hand.isEmpty()) {
                System.out.print("Error: No cards in hand\n");
                return false;
            } else if (src_depth != 1) {
                System.out.print("Error: Cannot take more than one card from hand\n");
                return false;
            }

            // Determine target area
            if (dest_pile > 6) {
                return this.move_hand_found(dest_pile);
            } else {
                return this.move_hand_pile(dest_pile);
            }
        }
            
            
        // Make sure that there is a card where we want to take from
        if (this.piles[src_pile].length < src_depth) {
            System.out.print("Error: Trying to take non-existant card\n");
            return false;
        }
        
        if dest_pile > 6 {
            game.move_pile_found(src_pile, src_depth, dest_pile)
        } else {
            game.move_pile_pile(src_pile, src_depth, dest_pile)
        }
    }

    public boolean move_hand_found(int dest_pile) {
        // Moving to a foundation:
        //   - Suit must match
        //   - number must be one higher
        Card card = this.hand.lastElement();
        if (!suit_match(dest_pile, card)) {
            System.out.print("Error: Suits must match on the foundations\n");
            return false;
        } else if (!number_match_asc(this.foundations[dest_pile - 7].length, card)) {
            System.out.print("Error: Numbers must ascend on the foundations\n");
            return false;
        }

        this.hand.remove(card);
        this.foundations[dest_pile - 7].push(card);
        this.score += 5;
        return true
    }


    public boolean move_hand_pile(game: &mut Game, dest_pile: usize) {
        // Moving to a pile:
        //   - suit must alternate
        //   - number must be one lower
        Card card = this.hand.lastElement();
        if (card.number == 13) {
            // King:
            //   - Destination pile must be empty
            if (!this.piles[dest_pile].isEmpty()) {
                System.out.print("Error: Kings can only be moved to empty piles\n");
                return false;
            }
        } else if (this.piles[dest_pile].isEmpty()) {
            System.out.print("Error: Only Kings can move to empty piles\n");
            return false;
        } else if (!suit_alternates(&card, this.piles[dest_pile].lastElement())) {
            System.out.print("Error: Suits must alternate on the piles\n");
            return false;
        } else if (!number_match_desc(this.piles[dest_pile].lastElement(), &card)) {
            System.out.print("Error: Numbers must decrease by one on the piles\n");
            return false;
        }
        
        this.hand.remove(card);
        this.piles[dest_pile].push(reveal(card));
        return true
    }

    public boolean move_pile_found(int src_pile, int src_depth, int dest_pile) {
        // Trying to move to a foundation:
        //   - Only one card allowed
        //   - Suit must match
        //   - Number must be one higher
        Card card = this.piles[src_pile][this.piles[src_pile].length - src_depth];
        if (src_depth != 1) {
            System.out.print("Error: Cannot move more than one card to foundation\n");
            return false;
        } else if (!suit_match(dest_pile, card)) {
            System.out.print("Error: Suits must match on the foundation\n");
            return false;
        } else if (!number_match_asc(game.foundations[dest_pile - 7].length, card)) {
            System.out.print("Error: Numbers must ascend by one on the foundation\n");
            return false;
        }

        // Validation has passed:
        this.piles[src_pile].remove(card);
        this.foundations[dest_pile - 7].push(card);
        // Make sure the last card in the pile is revealed if necessary
        if (this.piles[src_pile].length > 0) {
            this.piles[src_pile].last().up = true;
        }
        this.score += 5;
        return true
    }

    public boolean move_pile_pile(int src_pile, int src_depth, int dest_pile) {

        // Trying to move to another pile (we already know the source stack exists):

        Card card = this.piles[src_pile][game.piles[src_pile].length - src_depth];

        if (card.number == 13) {
            // King:
            //   - Destination pile must be empty
            if (!game.piles[dest_pile].isEmpty()) {
                System.out.print("Error: Kings can only be moved to empty piles\n");
                return false;
            }
        } else if (game.piles[dest_pile].isEmpty()) {
                System.out.print("Error: Cannot move non-king to empty pile\n");
                return false;
        } else if (!suit_alternates(card, game.piles[dest_pile].lastElement())) {
            System.out.print("Error: Suit colours must alternate on piles\n");
            return false;
        } else if (!number_match_desc(game.piles[dest_pile].last(), card)) {
            System.out.print("Error: Numbers must decrease by one on piles\n");
            return false;
        }

        // Validation has passed:
        int split_index = this.piles[src_pile].length - src_depth;

        // Split the array at that index:
        Vector<Card> cards = new Vector<Card>();
        int original_length = this.piles[src_pile].length;

        for (int i = split_index; i < original_length; i++) {
            // Note remove(split_index), remove(i) would skip elements (think about it)
            cards.add(0, this.piles[src_pile].remove(split_index));
        }

        // Now 'cards' should be everything that was on the pile after split_index

        this.piles[dest_pile].addAll(this.piles[dest_pile].size(), cards);
        if (!this.piles[src_pile].isEmpty()) {
            this.piles[src_pile].last().up = true;
        }
        return true
    }

    // Returns whether the hand was reset
    public boolean draw() {
        if (this.side_deck.isEmpty()) {
            this.side_deck = game.hand.clone();
            this.side_deck.reverse();
            this.hand = vec![];
            true
        } else {
            let mut moved = 0;
            while moved < 3 && !this.side_deck.isEmpty() {
                this.hand.push(reveal(&this.side_deck.pop().unwrap()));
                moved += 1;
            }
            false
        }
    }    
}

    public boolean suit_match(int dest, Card card) {
        if (dest == 7) {
            return card.suit == 'H';
        }
        if (dest == 8) {
            return card.suit == 'S';
        }
        if (dest == 9) {
            return card.suit == 'D';
        }
        if (dest == 10) {
            return card.suit == 'C';
        }
        return false;
    }

    public boolean suit_alternates(Card a, Card b) {
        if (a.suit == 'H' || a.suit == 'D') {
            return b.suit == 'C' || b.suit == 'S';
        }
        if (a.suit == 'C' || a.suit == 'S') {
            return b.suit == 'H' || b.suit == 'D';
        }
        return false;
    }

    public boolean number_match_asc(int base, Card card) {
        return card.number - 1 == base;
    }

    public boolean number_match_desc(Card dest, Card src) {
        if dest == null {
            return src.number == 13;
        } else {
            return dest.number - 1 == src.number,
        }
    }

    public Vector<Card> deck() {
        Vector<Card> deck = new Vector<Card>();

        char[] suits = ['H', 'D', 'C', 'S'];
        for (suit : suits) {
            for (int i = 1; i < 14; i++) { 
                deck.push(Card(suit, i));
            }
        }
        return deck;
    }
}

class Card {

    public char suit;
    public int  number;
    public boolean up;

    public Card(char suit, int number){
        this.suit = suit;
        this.number = number;
        this.up = false;
    }

    public Card clone() {
        Card card = Card(this.suit, this.number);
        card.up = this.up;
        return card;
    }

    public Card reveal(card: &Card) -> Card {
        Card card = Card(this.suit, this.number);
        card.up = true;
        return card;
    }

    public String str_disp() {
        if this.up { 
            return card_string(card) 
        } else { 
            return " XX" 
        }
    }

    public String card_string() {
        if this.number == 1 {
            return " A" + card.suit;
        } else if card.number == 11 {
            return " J{}" + card.suit;
        } else if card.number == 12 {
            return " Q{}" + card.suit;
        } else if card.number == 13 {
            return " K{}" + card.suit;
        } else {
            return "" + card.number + card.suit;
        }
    }

    public Vector<Card> shuffle(Vector<Card> deck) {
        Random rand = new Random();

        Vector<Card> res = new Vector<Card>();
        boolean[] used = new boolean[52];

        for (int card = 0; card < 52; card += 1) {
            int choice = rand.nextInt(52);
            while (used[choice]) {
                choice = rand.nextInt(52);
            }
            res.push(&deck[choice].clone());
            used[choice] = true;
        }

        return res;
    }
