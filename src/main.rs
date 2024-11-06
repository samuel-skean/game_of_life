use std::{thread::sleep, time::Duration};

use rand::{thread_rng, Rng};

// Fails when WIDTH or HEIGHT are bigger than ISIZE_MAX.
const WIDTH: usize = 20;
const HEIGHT: usize = 15;
type GridType = [[bool; WIDTH + 2]; HEIGHT + 2];
const BLANK_GRID: GridType = [[false; WIDTH + 2]; HEIGHT + 2];
const GENERATIONS: u64 = 50;

fn main() {
    let mut world_a = BLANK_GRID;
    let one_before_bottom_edge = world_a.len() - 1;
    // NOTE: If I remember correctly, the aliasing/liveness rules seemed like Rust
    // re-evaluated the expression after the `in`, at least in this case, with
    // slices. Maybe the slice itself implements the iterator trait? Does that
    // make for nicer semantics?
    for row in &mut world_a[1..one_before_bottom_edge] {
        let one_before_bottom_edge = row.len() - 1;
        thread_rng().fill(&mut row[1..one_before_bottom_edge]);
    }
    
    let mut world_b = BLANK_GRID;
    let mut old_world = &mut world_a;
    let mut new_world = &mut world_b;

    for gen in 0..GENERATIONS {
        println!("World at generation {gen}:\n{}", world_to_string(old_world));
        for i in 1..(WIDTH + 1) {
            for j in 1..(HEIGHT + 1) {
                new_world[j][i] = new_cell(old_world, i, j);
            }
        }
        (old_world, new_world) = (new_world, old_world);
        sleep(Duration::from_secs(2));
    }

    println!("Hello, world!");
}

fn count_neighbors(old_world: &GridType, x: usize, y: usize) -> u8 {
    let mut num_neighbors = 0u8;
    for i in x - 1..x + 1 {
        for j in y - 1..y + 1 {
            if i == x && j == y {
                continue;
            }
            if old_world[j][i] {
                num_neighbors += 1;
            }
        }
    }
    num_neighbors
}

fn new_cell(old_world: &GridType, x: usize, y: usize) -> bool {
    let old_cell = old_world[y][x];
    match count_neighbors(old_world, x, y) {
        0..=1 => false,
        2..=3 if old_cell => true,
        2 if !old_cell => false,
        3 if !old_cell => true,
        4..=8 => false,
        num_neighbors => panic!("Num neighbors was unexpectedly {num_neighbors}")
    }
}

fn world_to_string(world: &GridType) -> String {
    let mut visual = String::with_capacity((world.len() * world[0].len()) + world.len() /* newlines */);
    for row in world {
        for &cell in row {
            visual.push(if cell {'.'} else {'x'});
        }
        visual.push('\n');
    }
    visual
}