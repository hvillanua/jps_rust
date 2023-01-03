mod grid;
mod jps;
mod location;
pub mod tools;

use std::collections::HashSet;

use grid::Grid;
use location::Location;
fn main() {
    let walls = HashSet::from([
        Location { x: 5, y: 0 },
        Location { x: 5, y: 1 },
        Location { x: 2, y: 2 },
        Location { x: 5, y: 2 },
        Location { x: 2, y: 3 },
        Location { x: 5, y: 3 },
        Location { x: 2, y: 4 },
        Location { x: 5, y: 4 },
        Location { x: 2, y: 5 },
        Location { x: 4, y: 5 },
        Location { x: 5, y: 5 },
        Location { x: 6, y: 5 },
        Location { x: 7, y: 5 },
        Location { x: 2, y: 6 },
        Location { x: 2, y: 7 },
    ]);

    let map = Grid::new(10, 10, walls);

    let start = Location { x: 1, y: 1 };
    let goal = Location { x: 6, y: 2 };
    let came_from = jps::jps(&map, &start, &goal, tools::euclidean);
    let path = tools::reconstruct_path(&start, &goal, &came_from);

    tools::draw_grid(
        &map,
        None,
        None,
        &path,
        &came_from,
        Some(&start),
        Some(&goal),
    );
}
