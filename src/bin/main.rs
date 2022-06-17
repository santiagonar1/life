use life::game::Game;

fn main() {
    let coord_alive_cells = [(1, 2), (2, 1), (2, 2), (2, 4)];
    let mut game = Game::new(10, &coord_alive_cells);
    game.set_max_num_iterations(3).enable_plotting("rtest");
    game.play();
}
