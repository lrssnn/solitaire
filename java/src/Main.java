mod game;
mod player;

static int delay_ms = 0;
static int test_secs = 60 * 15;

class Main {

    public static void main(String[] args) {

        profile_silent();

        profile_stats_game();

        profile_stats_move();

        profile_full_print();

    }

    public void profile_silent() {
        let deck = deck();
        let mut deck = shuffle(&deck);
        let mut game = game_init();

        deal(&mut game, &mut deck);

        let mut player = player::create_player(game);

        while player.game.started.elapsed() <=  std::time::Duration::from_secs(test_secs) {
            if game_won(&mut player.game) {
                game_restart(&mut player.game);
                player::player_reset(&mut player);
            } else {
                if !player::play_one_move(&mut player) {
                    game_restart(&mut player.game);
                    player::player_reset(&mut player);
                }
            }
        }
        print_stats(&player.game);
        println!("");
        player
    }

    public void profile_stats_game() {
        let deck = deck();
        let mut deck = shuffle(&deck);
        let mut game = game_init();

        deal(&mut game, &mut deck);

        let mut player = player::create_player(game);

        while player.game.started.elapsed() < std::time::Duration::from_secs(test_secs) {
            if game_won(&mut player.game) {
                game_restart(&mut player.game);
                player::player_reset(&mut player);
            } else {
                if !player::play_one_move(&mut player) {
                    game_restart(&mut player.game);
                    player::player_reset(&mut player);
                    print_stats(&player.game);
                }
            }
        }
        println!("");
        player
    }

    public void profile_stats_move() {
        let deck = deck();
        let mut deck = shuffle(&deck);
        let mut game = game_init();

        deal(&mut game, &mut deck);

        let mut player = player::create_player(game);

        while player.game.started.elapsed() < std::time::Duration::from_secs(test_secs) {
            if game_won(&mut player.game) {
                game_restart(&mut player.game);
                player::player_reset(&mut player);
            } else {
                if !player::play_one_move(&mut player) {
                    game_restart(&mut player.game);
                    player::player_reset(&mut player);
                }
            }
            print_stats(&player.game);
        }
        println!("");
        player
    }

    public void profile_full_print() {
        term::initscr();
        term::start_color();
        term::init_color(COLOR_RED, 219*4, 51*4, 47*4);
        term::init_color(COLOR_BLK, 256*4, 256*4, 256*4);
        term::init_color(COLOR_BG, 0, 0, 0);
        term::init_pair(PAIR_RED, COLOR_RED, COLOR_BG);
        term::init_pair(PAIR_BLK, COLOR_BLK, COLOR_BG);

        let deck = deck();
        let mut deck = shuffle(&deck);
        let mut game = game_init();

        deal(&mut game, &mut deck);

        let mut player = player::create_player(game);

        while player.game.started.elapsed() < std::time::Duration::from_secs(test_secs) {
            if game_won(&mut player.game) {
                game_restart(&mut player.game);
                player::player_reset(&mut player);
            } else {
                if !player::play_one_move(&mut player) {
                    game_restart(&mut player.game);
                    player::player_reset(&mut player);
                }
            }
            print_game(&player.game);
        }
        player
    }

    public void delay() {
        wait_millis(delay_ms);
    }
}
