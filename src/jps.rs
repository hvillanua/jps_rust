use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use super::grid::Grid;
use super::location::Location;

type HeuristicFn = fn(&Location, &Location) -> f64;

fn jump(grid: &Grid, initial: &Location, dir: &Location, goal: &Location) -> Option<Location> {
    let new_loc = initial + dir;
    if !grid.is_valid_move(&initial, &dir) {
        return None;
    }
    if &new_loc == goal {
        return Some(new_loc);
    }
    for next in grid.pruned_neighbours(&new_loc, Some(*initial)) {
        if grid.is_forced(&next, &new_loc, &dir) {
            return Some(new_loc);
        }
    }
    if dir.x != 0 && dir.y != 0 {
        for new_dir in [Location { x: dir.x, y: 0 }, Location { x: 0, y: dir.y }] {
            let jump_point = jump(grid, &new_loc, &new_dir, goal);
            if jump_point.is_some() {
                return Some(new_loc);
            }
        }
    }
    jump(grid, &new_loc, dir, goal)
}

fn successors(
    grid: &Grid,
    current: &Location,
    parent: Option<Location>,
    goal: &Location,
) -> Vec<Location> {
    let mut successors: Vec<Location> = Vec::new();
    let neighbours = grid.pruned_neighbours(current, parent);
    for n in neighbours {
        let jump_point = jump(grid, current, &(n - current).direction(), goal);
        if jump_point.is_some() {
            successors.push(jump_point.unwrap());
        }
    }
    successors
}

#[derive(Copy, Clone, PartialEq)]
struct State {
    cost: f64,
    location: Location,
}

impl Eq for State {}

// Explicitly implement the trait so the queue becomes a min-heap instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .total_cmp(&self.cost)
            .then_with(|| self.location.cmp(&other.location))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn jps(
    grid: &Grid,
    start: &Location,
    goal: &Location,
    heuristic: HeuristicFn,
) -> HashMap<Location, Location> {
    let mut open_set: BinaryHeap<State> = BinaryHeap::new();
    let mut came_from: HashMap<Location, Location> = HashMap::new();
    let mut cost_so_far: HashMap<Location, f64> = HashMap::new();

    open_set.push(State {
        cost: 0.0,
        location: *start,
    });
    came_from.insert(*start, *start);
    cost_so_far.insert(*start, 0.0);
    let mut parent: Option<Location> = None;

    while let Some(State {
        cost: _,
        location: current,
    }) = open_set.pop()
    {
        if current == *goal {
            return came_from;
        }

        if &current != start {
            parent = Some(came_from[&current]);
        }

        for next in successors(grid, &current, parent, goal) {
            let new_cost = cost_so_far[&current] + heuristic(&current, &next);
            if !cost_so_far.contains_key(&next)
                || new_cost < *cost_so_far.get(&next).or(Some(&f64::INFINITY)).unwrap()
            {
                cost_so_far.insert(next, new_cost);
                came_from.insert(next, current);
                open_set.push(State {
                    cost: new_cost + heuristic(&next, goal),
                    location: next,
                });
            }
        }
    }
    came_from
}
