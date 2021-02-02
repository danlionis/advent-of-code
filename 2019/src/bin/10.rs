use std::collections::{HashMap, HashSet};
use std::f64;

fn main() {
    let input: String = std::fs::read_to_string("./input/input-10.txt").unwrap();

    let input: Vec<&str> = input.split("\n").collect();

    let mut asteroids: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in input.iter().enumerate() {
        for (x, a) in line.split_terminator("").skip(1).enumerate() {
            if a == "#" {
                asteroids.insert((x as i32, y as i32));
            }
        }
    }

    let mut result: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();

    for a1 in asteroids.iter().copied() {
        'inner: for a2 in asteroids.iter().copied() {
            if a1 == a2 {
                // do not check identical asteroids
                continue;
            }
            let dx = a2.0 - a1.0;
            let dy = a2.1 - a1.1;
            let step = gdc(dx, dy).abs();

            let dx = dx / step;
            let dy = dy / step;

            let mut x = a1.0;
            let mut y = a1.1;

            while x != a2.0 || y != a2.1 {
                x += dx;
                y += dy;

                if (x, y) == a2 {
                    break;
                }

                if asteroids.contains(&(x, y)) {
                    continue 'inner;
                }
            }

            // println!("match ({} {}) ({} {})", a1.0, a1.1, a2.0, a2.1);
            result.entry(a1).or_insert_with(HashSet::new).insert(a2);
        }

        // println!("\n");
        // break;
    }
    let max = result.clone().drain().max_by_key(|e| e.1.len()).unwrap();
    println!("part1: {}", max.1.len());

    let station = max.0;
    // let station = (11, 13);
    // let max = (station, result.get(&station).expect("no such asteroid"));
    dbg!(station);

    let mut asteroid_angles: Vec<((i32, i32), f64)> = max
        .1
        .iter()
        .filter(|&a| *a != station)
        .map(|&cord| {
            let mut angle = angle(station, cord) + f64::consts::FRAC_PI_2;
            // println!("{}", angle);
            if angle < 0.0 {
                angle += 2.0 * f64::consts::PI;
            }
            // angle += f64::consts::FRAC_PI_4;
            // println!("{}", angle);
            (cord, angle)
        })
        .collect();

    asteroid_angles.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for (i, a) in asteroid_angles.iter().enumerate() {
        let res = (a.0).0 * 100 + (a.0).1;
        println!(
            "{:>3}. ({:>2},{:>2}) {} \t\t {}",
            i + 1,
            (a.0).0,
            (a.0).1,
            a.1,
            res
        );
    }
}

fn gdc(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    };

    return gdc(b, a % b);
}

fn angle(a: (i32, i32), b: (i32, i32)) -> f64 {
    let dx: f64 = (b.0 - a.0).into();
    let dy: f64 = (b.1 - a.1).into();

    dy.atan2(dx)
}
