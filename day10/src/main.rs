use aoc::utils::get_lines;
use num::integer::gcd;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::f64::consts::PI;

fn count_visible_asteroids(map_external: &Vec<Vec<bool>>, i: i32, j: i32) -> i32 {
    let mut result = 0;

    let static_map = map_external.clone();
    let mut map = map_external.clone();

    let width: i32 = map[0].len() as i32;
    let height: i32 = map.len() as i32;
    for x in 0..height {
        for y in 0..width {
            if !static_map[x as usize][y as usize] {
                continue;
            }

            if x == i && j == y {
                continue;
            }

            let x_delta = x - i;
            let y_delta = y - j;
            let divisor = gcd(x_delta, y_delta);

            let xd_norm = x_delta / divisor;
            let yd_norm = y_delta / divisor;

            let mut step = divisor;
            loop {
                step += 1;

                let new_x = i + step * xd_norm;
                let new_y = j + step * yd_norm;

                if new_x < 0 || new_y < 0 || new_x >= height || new_y >= width {
                    break;
                }

                if static_map[new_x as usize][new_y as usize] {
                    map[new_x as usize][new_y as usize] = false;
                }
            }
        }
    }

    for x in 0..height {
        for y in 0..width {
            if map[x as usize][y as usize] {
                result += 1;
            }
        }
    }

    result - 1
}

struct Asteroid {
    x: i32,
    y: i32,
    theta: f32,
    dist: f32,
}

fn main() {
    let map_lines = get_lines("input.txt");
    let mut map: Vec<Vec<bool>> = Vec::new();

    let height = map_lines.len();
    let width = map_lines[0].len();
    map.resize(height, Vec::with_capacity(width));

    let mut laser_coordinates = (-1, -1);
    let mut asteroids: Vec<Asteroid> = Vec::new();

    // Part 1
    {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        for line in map_lines.iter() {
            for cell in line.chars() {
                map[x as usize].push(cell == '#');
            }

            x += 1;
        }

        let mut max_visible = 0;
        let mut max_coords = (-1, -1);
        for i in 0..height {
            for j in 0..width {
                if !map[i as usize][j as usize] {
                    continue;
                }

                let count = count_visible_asteroids(&map, i as i32, j as i32);
                if count > max_visible {
                    max_visible = count;
                    max_coords = (i as i32, j as i32);
                }
            }
        }

        println!("{}", max_visible);
        laser_coordinates = max_coords;
    }

    // Part 2
    {
        for i in 0..height {
            for j in 0..width {
                if !map[i as usize][j as usize] {
                    continue;
                }

                let dy = i as i32 - laser_coordinates.0;
                let dx = j as i32 - laser_coordinates.1;

                let dist = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

                let dot = dx as f32;
                let det = -dy as f32;
                let mut theta = dot.atan2(det);
                if theta < 0.0 {
                    theta += (PI + PI) as f32;
                }

                asteroids.push(Asteroid {
                    x: i as i32,
                    y: j as i32,
                    theta,
                    dist,
                });
            }
        }
    }

    asteroids.sort_by(|a, b| {
        let angle_cmp = a.theta.partial_cmp(&b.theta).unwrap_or(Ordering::Equal);
        if angle_cmp == Ordering::Equal {
            return a.dist.partial_cmp(&b.dist).unwrap_or(Ordering::Equal);
        }

        angle_cmp
    });

    let mut destroyed: Vec<bool> = vec![false; asteroids.len()];
    let mut num_destroyed = 0;
    'outer: loop {
        let mut last_theta: f32 = -1.0;
        for (i, asteroid) in asteroids.iter().cycle().enumerate() {
            let cycle_index = i % asteroids.len();
            if asteroid.x == laser_coordinates.0 && asteroid.y == laser_coordinates.1 {
                continue;
            }

            if destroyed[cycle_index] {
                continue;
            }

            if asteroid.theta == last_theta {
                continue;
            }

            destroyed[cycle_index] = true;
            last_theta = asteroid.theta;
            num_destroyed += 1;

            if num_destroyed == 200 {
                println!("{}", asteroid.y * 100 + asteroid.x);
                break 'outer;
            }
        }
    }
}
