use crate::universe::{Cell, Universe};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
pub fn test_tick_empty() {
    let mut universe = Universe::try_with_cells(vec![Cell::Dead; 24], 6).unwrap();
    let expected = Universe::try_with_cells(vec![Cell::Dead; 24], 6).unwrap();

    universe.tick();

    assert_eq!(
        universe.iter_cells().collect::<Vec<_>>(),
        expected.iter_cells().collect::<Vec<_>>(),
        "Empty universe should remain empty after tick"
    );
}

#[wasm_bindgen_test]
pub fn test_tick_block_still_life() {
    // Block pattern (still life)
    #[rustfmt::skip]
        let mut universe = Universe::try_with_cells(
            vec![
                Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
                Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Dead,
                Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Dead,
                Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
            ],
            4,
        ).unwrap();

    #[rustfmt::skip]
        let expected = Universe::try_with_cells(
            vec![
                Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
                Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Dead,
                Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Dead,
                Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
            ],
            4,
        ).unwrap();

    universe.tick();

    assert_eq!(
        universe.iter_cells().collect::<Vec<_>>(),
        expected.iter_cells().collect::<Vec<_>>(),
        "Block still life should remain unchanged after tick"
    );
}

#[wasm_bindgen_test]
pub fn test_tick_blinker_oscillator() {
    // Blinker pattern (oscillator)
    #[rustfmt::skip]
        let mut universe = Universe::try_with_cells(
            vec![
                Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
                Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Alive, Cell::Dead,
                Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
            ],
            5,
        ).unwrap();

    #[rustfmt::skip]
        let expected = Universe::try_with_cells(
            vec![
                Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
                Cell::Dead,  Cell::Dead,  Cell::Alive, Cell::Dead,  Cell::Dead,
                Cell::Dead,  Cell::Dead,  Cell::Alive, Cell::Dead,  Cell::Dead,
            ],
            5,
        ).unwrap();

    universe.tick();

    assert_eq!(
        universe.iter_cells().collect::<Vec<_>>(),
        expected.iter_cells().collect::<Vec<_>>(),
        "Blinker should oscillate after tick"
    );
}

#[wasm_bindgen_test]
pub fn test_tick_spaceship() {
    #[rustfmt::skip]
    let mut universe = Universe::try_with_cells(
        vec![
            Cell::Dead,  Cell::Dead,  Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Dead,
            Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Alive, Cell::Dead,  Cell::Dead,
            Cell::Dead,  Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
            Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
        ],
        6,
    ).unwrap();

    #[rustfmt::skip]
    let expected = Universe::try_with_cells(
        vec![
            Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
            Cell::Dead,  Cell::Alive, Cell::Dead,  Cell::Alive, Cell::Dead,  Cell::Dead,
            Cell::Dead,  Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
            Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
        ],
        6,
    ).unwrap();

    universe.tick();

    assert_eq!(
        universe.iter_cells().collect::<Vec<_>>(),
        expected.iter_cells().collect::<Vec<_>>(),
        "Universe did not match expected after tick"
    );
}
