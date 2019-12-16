use aoc::utils::get_lines;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Orbiter<'a> {
    name: &'a str,
    orbiting: Option<Rc<RefCell<Orbiter<'a>>>>,
    orbitedBy: Vec<Rc<RefCell<Orbiter<'a>>>>,
}

fn count(orbiter: &Rc<RefCell<Orbiter>>, comDistance: i32) -> i32 {
    let mut result = comDistance;
    let satellites: &Vec<Rc<RefCell<Orbiter>>> = &(orbiter.borrow()).orbitedBy;
    for orbiting in satellites.iter() {
        result += count(orbiting, comDistance + 1);
    }
    result
}

fn get_path_to_com<'a>(orbiter: &Rc<RefCell<Orbiter<'a>>>) -> Vec<&'a str> {
    let mut currentOrbiter: Rc<RefCell<Orbiter<'a>>> = Rc::clone(orbiter);
    let mut result = Vec::new();

    loop {
        let temp = Rc::clone(&currentOrbiter);
        let orbitingOption = &(temp.borrow()).orbiting;
        match orbitingOption {
            None => {
                break;
            }
            Some(o) => {
                currentOrbiter = Rc::clone(&o);
                result.push(o.borrow().name)
            }
        }
    }

    result
}

fn main() {
    let map = get_lines("input.txt").clone();
    let mut index = HashMap::<&str, Rc<RefCell<Orbiter>>>::new();

    index.insert(
        "COM",
        Rc::new(RefCell::new(Orbiter {
            name: "COM",
            orbitedBy: Vec::new(),
            orbiting: None,
        })),
    );

    for orbit in map.iter() {
        let orbiters: Vec<&str> = orbit.split(')').collect();

        if !index.contains_key(orbiters[0]) {
            index.insert(
                orbiters[0],
                Rc::new(RefCell::new(Orbiter {
                    name: orbiters[0],
                    orbitedBy: Vec::new(),
                    orbiting: None,
                })),
            );
        }

        if !index.contains_key(orbiters[1]) {
            index.insert(
                orbiters[1],
                Rc::new(RefCell::new(Orbiter {
                    name: orbiters[1],
                    orbitedBy: Vec::new(),
                    orbiting: None,
                })),
            );
        }

        let rightOrbiter = Rc::clone(index.get(orbiters[1]).unwrap());
        index
            .get(orbiters[0])
            .unwrap()
            .borrow_mut()
            .orbitedBy
            .push(rightOrbiter);

        let leftOrbiter = Rc::clone(index.get(orbiters[0]).unwrap());
        index.get(orbiters[1]).unwrap().borrow_mut().orbiting = Some(leftOrbiter);
    }

    // Part 1
    {
        println!("{}", count(index.get("COM").unwrap(), 0));
    }

    // Part 2
    {
        let santa_forward = get_path_to_com(index.get("SAN").unwrap()).clone();
        let us_forward = get_path_to_com(index.get("YOU").unwrap()).clone();

        let mut santa = santa_forward.iter().rev();
        let mut us = us_forward.iter().rev();

        let mut distance = 0;
        while (santa.next() == us.next()) {}

        while (santa.next() != None) {
            distance += 1;
        }

        while (us.next() != None) {
            distance += 1;
        }

        println!("{}", distance + 2);
    }
}
