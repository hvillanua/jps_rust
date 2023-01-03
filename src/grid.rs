use std::collections::HashSet;

use super::location::Location;

const ALL_DIRS: [Location; 8] = [
    /* East, West, North, South */
    /* NE, NW, SE, SW*/
    Location { x: 1, y: 0 },
    Location { x: -1, y: 0 },
    Location { x: 0, y: -1 },
    Location { x: 0, y: 1 },
    Location { x: 1, y: 1 },
    Location { x: -1, y: 1 },
    Location { x: 1, y: -1 },
    Location { x: -1, y: -1 },
];

pub struct Grid {
    width: u32,
    height: u32,
    pub walls: HashSet<Location>,
}

impl Grid {
    pub fn new(width: u32, height: u32, walls: HashSet<Location>) -> Self {
        Self {
            width,
            height,
            walls,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn is_in_bounds(&self, loc: &Location) -> bool {
        0 <= loc.x
            && loc.x < self.width.try_into().unwrap()
            && 0 <= loc.y
            && loc.y < self.height.try_into().unwrap()
    }

    pub fn is_passable(&self, loc: &Location) -> bool {
        self.walls.get(loc).is_none()
    }

    pub fn is_valid_move(&self, loc: &Location, dir: &Location) -> bool {
        let next_loc = &(loc + dir);
        if dir.x != 0 && dir.y != 0 {
            return self.is_in_bounds(next_loc)
                && self.is_passable(next_loc)
                && (self.is_passable(&(loc + Location { x: dir.x, y: 0 }))
                    || self.is_passable(&(loc + Location { x: 0, y: dir.y })));
        }
        self.is_in_bounds(&next_loc) && self.is_passable(&next_loc)
    }

    pub fn is_forced(&self, loc: &Location, parent: &Location, travel_dir: &Location) -> bool {
        let dir = (loc - parent).direction();
        // Diagonal move into diagonal check
        if travel_dir.x != 0 && travel_dir.y != 0 {
            if (dir.x == travel_dir.x && dir.y == -travel_dir.y)
                || (dir.x == -travel_dir.x && dir.y == travel_dir.y)
            {
                return true;
            }
        }
        // Horizontal or vertical move into diagonal check
        else if dir.x != 0 && dir.y != 0 {
            return true;
        }
        return false;
    }

    pub fn neighbours(&self, current: &Location, dirs: &[Location]) -> Vec<Location> {
        let mut results: Vec<Location> = Vec::new();
        for &dir in dirs {
            if self.is_valid_move(current, &dir) {
                results.push(current + dir);
            }
        }
        results
    }

    pub fn pruned_neighbours(&self, current: &Location, parent: Option<Location>) -> Vec<Location> {
        if parent.is_none() {
            return self.neighbours(current, &ALL_DIRS);
        }
        let parent = parent.unwrap();

        let mut current_neighbours: Vec<Location>;
        let dir = (current - parent).direction();
        let previous = &(current - dir);

        // Diagonal neighbour
        if dir.x != 0 && dir.y != 0 {
            let dir_x = Location { x: dir.x, y: 0 };
            let dir_y = Location { x: 0, y: dir.y };

            // Add natural neighbours
            current_neighbours = self.neighbours(current, &[dir, dir_x, dir_y]);

            // Add forced neighbours
            for candidate_dir in [&dir_x, &dir_y] {
                if !self.is_valid_move(previous, candidate_dir)
                    && self.is_valid_move(previous, &(2 * candidate_dir))
                {
                    current_neighbours.push(previous + 2 * candidate_dir);
                }
            }
        }
        // Horizontal or vertical neighbour
        else {
            // Add natural neighbours
            current_neighbours = self.neighbours(current, &[dir]);

            // Add forced neighbours
            let inverted_dir = &Location { x: dir.y, y: dir.x };
            if !self.is_valid_move(current, inverted_dir)
                && self.is_valid_move(current, &(inverted_dir + dir))
            {
                current_neighbours.push(current + inverted_dir + dir);
            }
            if !self.is_valid_move(current, &(-inverted_dir))
                && self.is_valid_move(current, &(-inverted_dir + dir))
            {
                current_neighbours.push(current - inverted_dir + dir);
            }
        }
        current_neighbours
    }
}
