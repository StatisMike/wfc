extern crate test;

use std::num::NonZeroU32;

use rand::{rngs::StdRng, SeedableRng};
use test::Bencher;
use wfc::orientation;
use wfc::{ForbidNothing, Size};
use wfc_image::{generate_image_with_rng, retry, ImagePatterns, WrapXY};

const EXAMPLE: &str = "../wfc-image/examples/rooms.png";

#[bench]
fn bench_gen_pattern_3x3(bencher: &mut Bencher) {
    let input_image = image::open(EXAMPLE).unwrap();
    let orientations = &[orientation::Orientation::Original];

    bencher.iter(|| {
        ImagePatterns::new(&input_image, NonZeroU32::new(3).unwrap(), orientations);
    })
}

#[bench]
fn bench_10x10_pattern_3x3(bencher: &mut Bencher) {
    let input_image = image::open(EXAMPLE).unwrap();
    let orientations = &[orientation::Orientation::Original];
    let output_size = Size::new(10, 10);

    bencher.iter(|| {
        let mut rng = StdRng::seed_from_u64(2137);

        generate_image_with_rng(
            &input_image,
            NonZeroU32::new(3).unwrap(),
            output_size,
            orientations,
            WrapXY,
            ForbidNothing,
            retry::NumTimes(0),
            &mut rng,
        )
        .unwrap();
    })
}

#[bench]
fn bench_10x10_pattern_4x4(bencher: &mut Bencher) {
    let input_image = image::open(EXAMPLE).unwrap();
    let orientations = &[orientation::Orientation::Original];
    let output_size = Size::new(10, 10);

    bencher.iter(|| {
        let mut rng = StdRng::seed_from_u64(21371);

        generate_image_with_rng(
            &input_image,
            NonZeroU32::new(4).unwrap(),
            output_size,
            orientations,
            WrapXY,
            ForbidNothing,
            retry::NumTimes(0),
            &mut rng,
        )
        .unwrap();
    })
}

#[bench]
fn bench_10x10_pattern_3x3_orientations_all(bencher: &mut Bencher) {
    let input_image = image::open(EXAMPLE).unwrap();
    let output_size = Size::new(10, 10);

    bencher.iter(|| {
        let mut rng = StdRng::seed_from_u64(2137);

        generate_image_with_rng(
            &input_image,
            NonZeroU32::new(3).unwrap(),
            output_size,
            &orientation::ALL,
            WrapXY,
            ForbidNothing,
            retry::NumTimes(0),
            &mut rng,
        )
        .unwrap();
    })
}

#[bench]
fn bench_20x20_pattern_3x3(bencher: &mut Bencher) {
    let input_image = image::open(EXAMPLE).unwrap();
    let orientations = &[orientation::Orientation::Original];
    let output_size = Size::new(20, 20);

    bencher.iter(|| {
        let mut rng = StdRng::seed_from_u64(2137);

        generate_image_with_rng(
            &input_image,
            NonZeroU32::new(3).unwrap(),
            output_size,
            orientations,
            WrapXY,
            ForbidNothing,
            retry::NumTimes(0),
            &mut rng,
        )
        .unwrap();
    })
}
