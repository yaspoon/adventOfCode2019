use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;
use std::cmp::PartialEq;
use std::cmp::Eq;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    l: i64,
}

impl PartialEq for Point {
    fn eq(&self , other: &Self) -> bool {
        return (self.x == other.x) && (self.y == other.y);
    }
}
impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn main() {
    let path = Path::new("input");

    let wires = read_input(&path);
    let mut wires_points: Vec<HashSet<Point>> = Vec::new();
    for wire in wires {
        let wire_points = wire_to_points(wire);
        wires_points.push(wire_points);
    }
    /*
    for (i,wire) in wires_points.iter().enumerate() {
        println!("wire {}", i);
        for path in wire {
            println!("path x:{}, y:{}, l:{}", path.x, path.y, path.l);
        }
    }
    */
    let mut count = 0;
    let mut min_distance = std::i64::MAX;
    for intersection in wires_points[0].intersection(&wires_points[1]) {
        count = count + 1;
        let point1 = match wires_points[0].get(&intersection) {
            Some(point) => point,
            None => panic!("Failed to find point x:{} y:{}", intersection.x, intersection.y),
        };
        let point2 = match wires_points[1].get(&intersection) {
            Some(point) => point,
            None => panic!("Failed to find point x:{} y:{}", intersection.x, intersection.y),
        };
        let distance = point1.l + point2.l;
        println!("Intersection: x:{} y:{} l:{}", intersection.x, intersection.y, distance);
        if distance < min_distance {
            min_distance = distance;
        }
    }
    if count > 0 {
        println!("min latency distance:{}", min_distance);
    } else {
        println!("No intersections found");
    }
}

fn iterate_wire_path(wire_points: &mut HashSet<Point>, start: Point, dir: (i64, i64), count: i64) -> Point {
    let mut new_point = start;
    for _i in 0..count {
        new_point.x = new_point.x + dir.0;
        new_point.y = new_point.y + dir.1;
        new_point.l = new_point.l + 1;
        wire_points.insert(new_point);
    }
    return new_point;
}

fn wire_to_points(wire: Vec<String>) -> HashSet<Point> {
    let mut wire_points: HashSet<Point> = HashSet::new();
    
    let mut point = Point {
        x: 0,
        y: 0,
        l: 0,
    };
    for path in wire {
        let dir = &path[0..1];
        let count: i64 = path[1..].parse().unwrap();
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
