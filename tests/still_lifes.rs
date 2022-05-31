use life::game::Game;

#[test]
fn aircraft_carrier() {
    // See: https://playgameoflife.com/lexicon/aircraft_carrier
    let expected_coord_alive_cells: Vec<(usize, usize)> =
        vec![(1, 1), (1, 2), (2, 1), (2, 4), (3, 3), (3, 4)];

    let mut game = Game::new(6, &expected_coord_alive_cells);

    assert_eq!(expected_coord_alive_cells, game.coord_alive_cells());

    game.next();
    assert_eq!(expected_coord_alive_cells, game.coord_alive_cells());
}
