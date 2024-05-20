use std::num::NonZeroU32;

use grid_2d::Grid;
use rand::{distributions::Uniform, rngs::StdRng, SeedableRng};
use utils::generate_input_grid;
use wfc::{
    orientation, overlapping::OverlappingPatterns, retry, wrap::WrapXY, Context, Coord,
    RunBorrow, Size, Wave,
};

mod utils;

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 10;
const RETRY_TIMES: usize = 10;

fn main() {
    // Let us generate a 10x10 input grid of u32.
    let input =
        generate_input_grid::<u32>(GRID_WIDTH, GRID_HEIGHT, Uniform::new(0, 5), 2137);

    // Create the patterns for the algorithm to generate new object.
    let patterns = OverlappingPatterns::new(
        input.clone(),
        // Patterns will be 3x3 tiles.
        NonZeroU32::new(3).unwrap(),
        // Patterns will be generated in all orientations.
        &orientation::ALL,
    );

    let global_stats = patterns.global_stats();

    // Seed for reproductability.
    let mut rng = StdRng::seed_from_u64(2137);

    // Create data containers and WFC runner which borrows the data.
    let mut context = Context::new();
    let mut wave = Wave::new(Size::new(GRID_WIDTH as u32, GRID_HEIGHT as u32));

    let mut runner: RunBorrow<WrapXY, wfc::ForbidNothing> =
        RunBorrow::new(&mut context, &mut wave, &global_stats, &mut rng);

    // Collapse the WFC to generate new grid, retrying up to 10 times if met with contradiction.
    runner
        .collapse_retrying(retry::NumTimes(RETRY_TIMES), &mut rng)
        .unwrap_or_else(|_| panic!("cannot resolve after {RETRY_TIMES} retries"));

    // Construct output grid.
    //
    // The `Wave` consists of `WaveCells`, which hold information about the pattern chosen for given grid position.
    // Each of patterns give us information about `top_left_value`, which is the value that will be present in our
    // output grid.
    let mut output = Grid::new_default(wave.grid().size());

    wave.grid().enumerate().for_each(|(Coord { x, y }, cell)| {
        if let Ok(chosed_pattern_id) = cell.chosen_pattern_id() {
            *output.get_mut(Coord { x, y }).unwrap() =
                *patterns.pattern_top_left_value(chosed_pattern_id);
        }
    });

    // Output and input grids are different.
    assert_ne!(input, output);
}
