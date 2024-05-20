use grid_2d::Grid;
use rand::{
    distributions::{uniform::SampleUniform, Distribution, Uniform},
    rngs::StdRng,
    SeedableRng,
};
use wfc::{Coord, Size};

pub fn generate_input_grid<T: SampleUniform + Default>(
    width: usize,
    height: usize,
    options: Uniform<T>,
    seed: u64,
) -> Grid<T> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut grid = Grid::new_default(Size::new(width as u32, height as u32));

    for x in 0..width {
        for y in 0..height {
            let cell = grid.get_mut(Coord::new(x as i32, y as i32)).unwrap();
            *cell = options.sample(&mut rng);
        }
    }

    grid
}
