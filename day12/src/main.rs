use aoc::utils::get_lines;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use num::integer::lcm;

#[derive(Hash, Eq, PartialEq, Clone)]
struct Planet {
    id: i32,
    x: i32,
    y: i32,
    z: i32,
    velocity: Velocity,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}


fn all_x_match(planets: &HashMap<i32, Planet>, initial_state: &HashMap<i32, Planet>) -> bool {
    for (id, planet) in planets {
        let initial_planet = initial_state.get(id).unwrap();
        if initial_planet.x != planet.x {
            return false;
        }

        if initial_planet.velocity.x != planet.velocity.x {
            return false;
        }
    }

    true
}

fn all_y_match(planets: &HashMap<i32, Planet>, initial_state: &HashMap<i32, Planet>) -> bool {
    for (id, planet) in planets {
        let initial_planet = initial_state.get(id).unwrap();
        if initial_planet.y != planet.y {
            return false;
        }

        if initial_planet.velocity.y != planet.velocity.y {
            return false;
        }
    }

    true
}

fn all_z_match(planets: &HashMap<i32, Planet>, initial_state: &HashMap<i32, Planet>) -> bool {
    for (id, planet) in planets {
        let initial_planet = initial_state.get(id).unwrap();
        if initial_planet.z != planet.z {
            return false;
        }

        if initial_planet.velocity.z != planet.velocity.z {
            return false;
        }
    }

    true
}

fn update_state(planets: &HashMap<i32, Planet>) -> HashMap<i32, Planet> {
    let mut updated_planets: HashMap<i32, Planet> = planets.clone();
    for (id, planet) in planets.iter() {
        let mut updated_velocity = planet.velocity.clone();
        for (other_id, other_planet) in planets.iter() {
            if id == other_id {
                continue;
            }

            updated_velocity.x += match planet.x == other_planet.x {
                true => 0,
                false => match planet.x < other_planet.x {
                    true => 1,
                    false => -1,
                },
            };

            updated_velocity.y += match planet.y == other_planet.y {
                true => 0,
                false => match planet.y < other_planet.y {
                    true => 1,
                    false => -1,
                },
            };

            updated_velocity.z += match planet.z == other_planet.z {
                true => 0,
                false => match planet.z < other_planet.z {
                    true => 1,
                    false => -1,
                },
            };
        }

        let updated_planet = Planet {
            id: *id,
            x: updated_velocity.x + planet.x,
            y: updated_velocity.y + planet.y,
            z: updated_velocity.z + planet.z,
            velocity: updated_velocity,
        };

        updated_planets.insert(*id, updated_planet);
    }

    updated_planets
}

fn main() {
    let lines = get_lines("input.txt").clone();

    let mut planets: HashMap<i32, Planet> = HashMap::new();
    let initial_state: HashMap<i32, Planet>;

    let steps = 1000;
    // Part 1
    {
        let mut id = 0;
        for line in lines.iter() {
            id += 1;
            if let Ok((x, y, z)) = scan_fmt!(line, "<x={d}, y={d}, z={d}>", i32, i32, i32) {
                let velocity = Velocity { x: 0, y: 0, z: 0 };
                planets.insert(
                    id,
                    Planet {
                        id,
                        x,
                        y,
                        z,
                        velocity,
                    },
                );
            }
        }

        initial_state = planets.clone();

        for i in 0..steps {
            planets = update_state(&planets);
        }

        let mut total_energy = 0;
        for (_, planet) in planets.iter() {
            total_energy += (planet.x.abs() + planet.y.abs() + planet.z.abs())
                * (planet.velocity.x.abs() + planet.velocity.y.abs() + planet.velocity.z.abs());
        }

        println!("{}", total_energy);
    }

    // Part 2
    {
        let mut period_x = -1;
        let mut period_y = -1;
        let mut period_z = -1;

        let mut i = 0;
        planets = initial_state.clone();
        while period_x < 0 {
            if (i != 0 && all_x_match(&planets, &initial_state)) {
                period_x = i;
            } else {
                i += 1;
                planets = update_state(&planets);
            }
        }

        let mut i = 0;
        planets = initial_state.clone();
        while period_y < 0 {
            if (i != 0 && all_y_match(&planets, &initial_state)) {
                period_y = i;
            } else {
                i += 1;
                planets = update_state(&planets);
            }
        }

        let mut i = 0;
        planets = initial_state.clone();
        while period_z < 0 {
            if (i != 0 && all_z_match(&planets, &initial_state)) {
                period_z = i;
            } else {
                i += 1;
                planets = update_state(&planets);
            }
        }

        println!("lcm({}, {}, {}) = {}", period_x, period_y, period_z, lcm(period_x as i64, lcm(period_y as i64, period_z as i64)));
    }
}
