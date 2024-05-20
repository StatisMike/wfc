extern crate test;

use std::num::NonZeroU32;

use grid_2d::Grid;
use rand::{rngs::StdRng, SeedableRng};
use test::Bencher;
use wfc::{
    overlapping::OverlappingPatterns, Context, Coord, RunBorrow, RunOwn, Size, Wave,
};
use wfc_image::WrapXY;

const TEST_GRID_10X10: [[u8; 10]; 10] = [
    [0, 1, 1, 2, 0, 1, 1, 2, 2, 1],
    [1, 2, 2, 2, 1, 3, 1, 1, 1, 1],
    [0, 1, 2, 2, 2, 1, 3, 1, 1, 1],
    [0, 1, 1, 2, 2, 2, 1, 3, 1, 1],
    [0, 0, 1, 1, 0, 1, 1, 0, 0, 1],
    [0, 1, 1, 1, 0, 1, 1, 0, 0, 1],
    [0, 1, 1, 2, 0, 1, 1, 2, 2, 1],
    [2, 0, 1, 1, 2, 0, 1, 1, 2, 1],
    [1, 1, 1, 1, 2, 2, 2, 1, 1, 1],
    [2, 2, 2, 2, 1, 2, 2, 2, 2, 1],
];

fn generate_input_grid<const WIDTH: usize, const HEIGHT: usize>(
    data: &[[u8; WIDTH]; HEIGHT],
) -> Grid<u8> {
    let mut input = Grid::<u8>::new_default(Size::new(WIDTH as u32, HEIGHT as u32));
    for (y, row) in data.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            let cell = input.get_mut(Coord::new(x as i32, y as i32)).unwrap();
            *cell = *val;
        }
    }
    input
}

#[bench]
fn bench_gen_pattern_3x3_from_10x10(bencher: &mut Bencher) {
    let input = generate_input_grid(&TEST_GRID_10X10);

    bencher.iter(|| {
        OverlappingPatterns::new_original_orientation(
            input.clone(),
            NonZeroU32::new(3).unwrap(),
        );
    })
}

#[bench]
fn bench_10x10_pattern_3x3_borrow(bencher: &mut Bencher) {
    let input = generate_input_grid(&TEST_GRID_10X10);

    let global_stats = OverlappingPatterns::new_original_orientation(
        input.clone(),
        NonZeroU32::new(3).unwrap(),
    )
    .global_stats();

    bencher.iter(|| {
        let mut rng = StdRng::seed_from_u64(21371);

        let mut wave = Wave::new(Size::new(10, 10));
        let mut context = Context::new();

        let mut run =
            RunBorrow::new_wrap(&mut context, &mut wave, &global_stats, WrapXY, &mut rng);

        run.collapse(&mut rng).unwrap();
    });
}

#[bench]
fn bench_20x20_pattern_3x3_borrow(bencher: &mut Bencher) {
    let input = generate_input_grid(&TEST_GRID_10X10);

    let global_stats = OverlappingPatterns::new_original_orientation(
        input.clone(),
        NonZeroU32::new(3).unwrap(),
    )
    .global_stats();

    bencher.iter(|| {
        let mut rng = StdRng::seed_from_u64(21371);

        let mut wave = Wave::new(Size::new(20, 20));
        let mut context = Context::new();

        let mut run =
            RunBorrow::new_wrap(&mut context, &mut wave, &global_stats, WrapXY, &mut rng);

        run.collapse(&mut rng).unwrap();
    });
}

#[bench]
fn bench_10x10_pattern_3x3_own(bencher: &mut Bencher) {
    let input = generate_input_grid(&TEST_GRID_10X10);

    let global_stats = OverlappingPatterns::new_original_orientation(
        input.clone(),
        NonZeroU32::new(3).unwrap(),
    )
    .global_stats();

    bencher.iter(|| {
        let mut rng = StdRng::seed_from_u64(21371);

        let mut run =
            RunOwn::new_wrap(Size::new(10, 10), &global_stats, WrapXY, &mut rng);

        run.collapse(&mut rng).unwrap();
    });
}
