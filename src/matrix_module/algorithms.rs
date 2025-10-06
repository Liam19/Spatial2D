use crate::*;

use core::{
    cmp::{Ordering, Reverse},
    f32::consts::SQRT_2,
};
use priority_queue::PriorityQueue;
use std::collections::VecDeque;

impl<T> Matrix<T> {
    /// Performs a breadth-first-search until search_fn returns true
    pub fn bfs(
        &self,
        starting_pos: UVec2,
        search_fn: impl Fn(&T, UVec2) -> bool,
    ) -> Option<(UVec2, &T)> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(starting_pos);
        visited.insert(starting_pos);

        while let Some(pos) = queue.pop_front() {
            let value = self.get(pos);

            if search_fn(self.get(pos), pos) {
                return Some((pos, value));
            }

            for neighbour in self.neighbours_no_diag(pos) {
                if visited.insert(neighbour) {
                    queue.push_back(neighbour);
                }
            }
        }

        None
    }

    /// Performs a breadth-first-search until there are no more positions to search.
    ///
    /// Returns all positions where search_fn returned true
    pub fn bfs_multi(
        &self,
        starting_pos: UVec2,
        search_fn: impl Fn(&T, UVec2) -> bool,
    ) -> Vec<(UVec2, &T)> {
        let mut found = Vec::new();

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(starting_pos);
        visited.insert(starting_pos);

        while let Some(pos) = queue.pop_front() {
            let value = self.get(pos);

            if search_fn(self.get(pos), pos) {
                found.push((pos, value));

                for neighbour in self.neighbours_no_diag(pos) {
                    if visited.insert(neighbour) {
                        queue.push_back(neighbour);
                    }
                }
            }
        }

        found
    }

    /// Performs a breadth-first-search until there are no more positions to search.
    ///
    /// Returns all positions where search_fn returned true
    pub fn bfs_multi_with_diag(
        &self,
        starting_pos: UVec2,
        search_fn: impl Fn(&T, UVec2) -> bool,
    ) -> Vec<(UVec2, &T)> {
        let mut found = Vec::new();

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(starting_pos);
        visited.insert(starting_pos);

        while let Some(pos) = queue.pop_front() {
            let value = self.get(pos);

            if search_fn(self.get(pos), pos) {
                found.push((pos, value));

                for neighbour in self.neighbours(pos) {
                    if visited.insert(neighbour) {
                        queue.push_back(neighbour);
                    }
                }
            }
        }

        found
    }

    /// Performs A* pathfinding between two points on the grid
    ///
    /// ## Arguments
    /// * `start` - Starting position
    /// * `target` - Target position
    /// * `move_speed_fn` - Calculation for the move speed when going from current position to a neighbouring position
    /// (current_pos, neighbour_pos, T)
    /// * `turn_penalty` - Added to the cost of turning a corner
    /// * `diag_cost_multiplier` - None for no diagonal movement
    ///
    /// ## Returns
    /// * `Some(Vec<UVec2>)` - A valid path if one is found
    /// * `None` - If no path could be found or search depth exceeded
    pub fn a_star_search<'a>(
        &'a self,
        start: UVec2,
        target: UVec2,
        move_speed_fn: impl Fn(UVec2, UVec2, &'a T) -> f32,
        turn_penalty: f32,
        max_search_depth: Option<u32>,
        diag_cost_multiplier: Option<f32>,
    ) -> Option<Vec<UVec2>> {
        let element_count = self.element_count() as usize;

        // Flat arrays for storage
        let mut came_from = vec![UVec2::MAX; element_count];
        let mut g_costs: Vec<f32> = vec![f32::INFINITY; element_count];
        let mut closed: Vec<bool> = vec![false; element_count];
        let mut open_nodes: PriorityQueue<UVec2, Reverse<FloatOrd>> = PriorityQueue::new();

        // Priority queue using Reverse for min-heap behavior
        open_nodes.push(start, Reverse(FloatOrd(0.0)));

        // Initialize starting position
        let start_idx = self.pos_to_idx(start) as usize;
        g_costs[start_idx] = 0.0;

        let mut current_depth = 0;
        let max_depth = max_search_depth.unwrap_or(u32::MAX);
        let diag_m = diag_cost_multiplier.unwrap_or(1.0);

        while let Some((current_pos, _)) = open_nodes.pop() {
            current_depth += 1;

            // Early exit if we've exceeded our search depth
            if current_depth > max_depth {
                println!("Max search depth exceeded ({current_depth} nodes)");

                return None;
            }

            // Found our target
            if current_pos == target {
                return Some(self.retrace_path(&came_from, current_pos));
            }

            let current_idx = self.pos_to_idx(current_pos) as usize;
            closed[current_idx] = true;

            // Get neighbors based on movement rules
            let neighbors = if diag_cost_multiplier.is_some() {
                self.neighbours(current_pos)
            } else {
                self.neighbours_no_diag(current_pos)
            };

            for neighbour_pos in neighbors {
                let neighbour_idx = self.pos_to_idx(neighbour_pos) as usize;

                // Skip if already closed
                if closed[neighbour_idx] {
                    continue;
                }

                // Calculate movement cost
                let move_speed = move_speed_fn(current_pos, neighbour_pos, self.get(neighbour_pos));
                if move_speed <= 0.0 {
                    continue;
                }

                let move_cost = 1.0 / move_speed;

                // Check if neighbour requires a turn
                let mut is_turn = false;

                if let Some(&previous_pos) = came_from.get(current_idx) {
                    // MAX is our sentinel value
                    if previous_pos != UVec2::MAX {
                        let prev_dir = current_pos.as_ivec2() - previous_pos.as_ivec2();
                        let new_dir = neighbour_pos.as_ivec2() - current_pos.as_ivec2();

                        is_turn = prev_dir != new_dir;
                    }
                }

                let tentative_g_cost =
                    g_costs[current_idx] + move_cost + if is_turn { turn_penalty } else { 0.0 };

                // Found a better path to this neighbor
                if tentative_g_cost < g_costs[neighbour_idx] {
                    came_from[neighbour_idx] = current_pos;
                    g_costs[neighbour_idx] = tentative_g_cost;

                    // // Calculate heuristic (using Chebyshev distance for diagonal movement)
                    // let h_cost = if allow_diagonal {
                    //     chebyshev_distance(neighbour_pos, target)
                    // } else {
                    //     manhattan_distance(neighbour_pos, target)
                    // };

                    let h_cost = calculate_h_cost(neighbour_pos, target, diag_m);

                    let f_cost = tentative_g_cost + h_cost;

                    // Update or insert in priority queue
                    open_nodes.push(neighbour_pos, Reverse(FloatOrd(f_cost)));
                }
            }
        }

        None
    }

    /// Optimized path retracing using flat array
    fn retrace_path(&self, came_from: &[UVec2], mut current: UVec2) -> Vec<UVec2> {
        let mut path = Vec::new();
        path.push(current);

        let mut current_idx = self.pos_to_idx(current) as usize;

        while came_from[current_idx] != UVec2::MAX {
            current = came_from[current_idx];
            current_idx = self.pos_to_idx(current) as usize;
            path.push(current);
        }

        path.reverse();

        path
    }
}

// /// Chebyshev distance (for diagonal movement)
// #[inline]
// fn chebyshev_distance(a: UVec2, b: UVec2) -> f32 {
//     let dx = (a.x as i32 - b.x as i32).abs() as f32;
//     let dy = (a.y as i32 - b.y as i32).abs() as f32;

//     dx.max(dy)
// }

// /// Manhattan distance (for non-diagonal movement)
// #[inline]
// fn manhattan_distance(a: UVec2, b: UVec2) -> f32 {
//     let dx = (a.x as i32 - b.x as i32).abs() as f32;
//     let dy = (a.y as i32 - b.y as i32).abs() as f32;

//     dx + dy
// }

/// Gets the distance of the straightest path between two positions
///
/// The path must contain only horizontal, vertical, and diagonal movements
#[inline]
fn calculate_h_cost(start_pos: UVec2, target_pos: UVec2, diag_speed_multiplier: f32) -> f32 {
    let s_pos = start_pos.as_ivec2();
    let t_pos = target_pos.as_ivec2();

    let dist_x = (s_pos.x - t_pos.x).abs() as f32;
    let dist_y = (s_pos.y - t_pos.y).abs() as f32;

    let v = if dist_x > dist_y {
        (SQRT_2 * diag_speed_multiplier).mul_add(dist_y, 1.0 * (dist_x - dist_y))
    } else {
        (SQRT_2 * diag_speed_multiplier).mul_add(dist_x, 1.0 * (dist_y - dist_x))
    };

    v

    // if v < 0.0 {
    //     dbg!(start_pos, target_pos);
    //     dbg!(dist_x, dist_y);
    //     dbg!(v);

    //     panic!("h_cost should not be negative");
    // } else {
    //     v
    // }
}

/// Wrapper for floating point comparisons in priority queue
#[derive(PartialEq, PartialOrd)]
struct FloatOrd(f32);

impl Eq for FloatOrd {}

impl Ord for FloatOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.partial_cmp(&self.0).unwrap_or(Ordering::Equal)
    }
}
