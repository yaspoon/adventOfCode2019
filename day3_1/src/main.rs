use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;
//use std::convert::TryInto;

fn main() {
    let path = Path::new("input");

    let wires = read_input(&path);
    let mut wires_points: Vec<HashSet<(i32, i32)>> = Vec::new();
    for wire in wires {
        let wire_points = wire_to_points(wire);
        wires_points.push(wire_points);
    }
    for (i,wire) in wires_points.iter().enumerate() {
        println!("wire {}", i);
        for path in wire {
            println!("path x:{}, y:{}", path.0, path.1);
        }
    }
    let mut min_distance = std::i32::MAX;
    for point in wires_points[0].intersection(&wires_points[1]) {
        println!("Intersection: x:{} y:{}", point.0, point.1);
        let x_distance: f32 = point.0 as f32;
        let y_distance: f32 = point.1 as f32;
        let distance: i32 = (x_distance.powf(2.0).sqrt() + y_distance.powf(2.0).sqrt()) as i32;
        if distance < min_distance {
            min_distance = distance;
        }
    }
    println!("min distance:{}", min_distance);
}

fn iterate_wire_path(wire_points: &mut HashSet<(i32, i32)>, start: (i32, i32), dir: (i32, i32), count: i32) -> (i32, i32) {
    let mut new_point = start;
    for _i in 0..count {
        new_point = (new_point.0 + dir.0, new_point.1 + dir.1);
        wire_points.insert(new_point);
    }
    return new_point;
}

fn wire_to_points(wire: Vec<String>) -> HashSet<(i32, i32)> {
    let mut wire_points: HashSet<(i32, i32)> = HashSet::new();
    
    let mut point: (i32, i32) = (0, 0);
    for path in wire {
        let dir = &path[0..1];
        let count: i32 = path[1..].parse().unwrap();
        //println!("{}", dir);
        match dir {
            "R" => point = iterate_wire_path(&mut wire_points, point, (1, 0), count),
            "L" => point = iterate_wire_path(&mut wire_points, point, (-1, 0), count),
            "U" => point = iterate_wire_path(&mut wire_points, point, (0, 1), count),
            "D" => point = iterate_wire_path(&mut wire_points, point, (0, -1), count),
            _ => panic!("Unknown direction"),
        };
    }
    return wire_points;
}

fn read_input(path: &Path) -> Vec<Vec<String>> {
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).expect("Failed to read input");

    let mut vec: Vec<Vec<String>> = Vec::new();
    for line in s.lines() {
        let mut wire: Vec<String> = Vec::new();
        for path in line.split(',') {
            wire.push(path.to_string());
        }
        vec.push(wire);
    }

    return vec;
}
