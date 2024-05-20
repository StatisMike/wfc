use std::num::NonZeroU32;

use grid_2d::Grid;
use rand::{distributions::Uniform, rngs::StdRng, SeedableRng};
use utils::generate_input_grid;
use wfc::{
    orientation, overlapping::OverlappingPatterns, wrap::WrapXY, Coord, Observe, RunOwn,
    Size,
};

mod utils;

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 10;

fn main() {
    // Let us generate a 10x10 input grid of u32.
    let input =
        generate_input_grid::<u32>(GRID_WIDTH, GRID_HEIGHT, Uniform::new(0, 3), 2137);

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

    // Create WFC runner which owns the data.
    let mut runner: RunOwn<WrapXY, wfc::ForbidNothing> = RunOwn::new(
        Size::new(GRID_WIDTH as u32, GRID_HEIGHT as u32),
        &global_stats,
        &mut rng,
    );

    // To keep track of `WaveCells` that have been collapsed.
    let mut collapsed_coords = Vec::<Coord>::new();

    // Collapse step-by-step, printing the collapsed tiles
    loop {
        match runner.step(&mut rng) {
            Ok(observe) => {
                for (coord, cell) in runner.wave_cell_ref_enumerate() {
                    if let Ok(collapsed_pattern) = cell.as_ref().chosen_pattern_id() {
                        if !collapsed_coords.contains(&coord) {
                            let collapsed_value =
                                patterns.pattern_top_left_value(collapsed_pattern);
                            println!(
                                "collapsed value: {collapsed_value} at coord: {coord:?}"
                            );
                            collapsed_coords.push(coord);
                        }
                    }
                }
                if observe == Observe::Complete {
                    break;
                }
            }
            Err(err) => {
                panic!("propagation error!: {err:?}");
            }
        }
    }

    // Collapse the WFC to generate new grid, retrying up to 10 times if met with contradiction.
    let wave = runner.into_wave();

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
