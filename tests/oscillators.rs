use life::game::Game;

#[test]
fn blinker() {
    // See: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
    let expected_coord_alive_cells: Vec<(usize, usize)> = vec![(1, 2), (2, 2), (3, 2)];

    let mut game: Game = Game::new(5, &expected_coord_alive_cells);

    assert_eq!(expected_coord_alive_cells, game.coord_alive_cells());

    game.next();

    let expected_coord_alive_cells: Vec<(usize, usize)> = vec![(2, 1), (2, 2), (2, 3)];
    assert_eq!(expected_coord_alive_cells, game.coord_alive_cells());
}
