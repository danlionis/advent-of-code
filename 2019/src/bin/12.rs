fn main() {
    // <x=13, y=9, z=5>
    // <x=8, y=14, z=-2>
    // <x=-5, y=4, z=11>
    // <x=2, y=-6, z=1>

    let p1 = Planet::new(13, 9, 5);
    let p2 = Planet::new(8, 14, -2);
    let p3 = Planet::new(-5, 4, 11);
    let p4 = Planet::new(2, -6, 1);

    // <x=-1, y=0, z=2>
    // <x=2, y=-10, z=-7>
    // <x=4, y=-8, z=8>
    // <x=3, y=5, z=-1>

    // let p1 = Planet::new(-1, 0, 2);
    // let p2 = Planet::new(2, -10, -7);
    // let p3 = Planet::new(4, -8, 8);
    // let p4 = Planet::new(3, 5, -1);

    let mut planets: Vec<Planet> = vec![p1, p2, p3, p4];

    let initial = planets.clone();

    let mut repeat_x = None;
    let mut repeat_y = None;
    let mut repeat_z = None;

    let mut i = 0;

    while repeat_x.is_none() || repeat_y.is_none() || repeat_z.is_none() {
        if i % 100000 == 0 {
            println!("i: {}", i);
        }
        planets = planets
            .iter()
            .map(|p1| {
                let mut p = p1.clone();
                for p2 in planets.iter() {
                    if p1.pos.x > p2.pos.x {
                        p.vel.x -= 1;
                    } else if p1.pos.x < p2.pos.x {
                        p.vel.x += 1;
                    }

                    if p1.pos.y > p2.pos.y {
                        p.vel.y -= 1;
                    } else if p1.pos.y < p2.pos.y {
                        p.vel.y += 1;
                    }

                    if p1.pos.z > p2.pos.z {
                        p.vel.z -= 1;
                    } else if p1.pos.z < p2.pos.z {
                        p.vel.z += 1;
                    }
                }
                p.move_by_vel();
                p
            })
            .collect();
        i += 1;

        if planets.iter().enumerate().all(|(ip, p)| {
            let init = &initial[ip];
            p.pos.x == init.pos.x && p.vel.x == init.vel.x
        }) {
            println!("found x: {}", i);
            repeat_x = repeat_x.or(Some(i));
        }

        if planets.iter().enumerate().all(|(ip, p)| {
            let init = &initial[ip];
            p.pos.y == init.pos.y && p.vel.y == init.vel.y
        }) {
            println!("found y: {}", i);
            repeat_y = repeat_y.or(Some(i));
        }

        if planets.iter().enumerate().all(|(ip, p)| {
            let init = &initial[ip];
            p.pos.z == init.pos.z && p.vel.z == init.vel.z
        }) {
            println!("found z: {}", i);
            repeat_z = repeat_z.or(Some(i));
        }

        // planets.iter().for_each(|p| {
        //     dbg!(&p);
        // });
    }

    dbg!(repeat_x, repeat_y, repeat_z);

    let tmp = lcm(repeat_x.unwrap(), repeat_y.unwrap());
    let res = lcm(tmp, repeat_z.unwrap());

    println!("part2: {}", res);

    // let part1 = planets.iter().map(|p| p.energy()).fold(0, |a, b| a + b);
    // println!("part1: {}", part1);

    // for i in 0.. {
    //     for i1 in 0..planets.len() {
    //         // let mut p1 = planets.get_mut(i1).unwrap();
    //         let p1 = &planets[i1];
    //         for i2 in 0..planets.len() {
    //             let p2 = &planets[i2];
    //             if p1.pos.x > p2.pos.y {
    //                 planets[i].vel.x += 1;
    //             }
    //         }
    //     }
    // }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gdc(a, b)
}

fn gdc(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    };

    return gdc(b, a % b);
}

#[derive(Clone, Debug, Hash)]
struct Planet {
    pos: Vec3,
    vel: Vec3,
}

impl Planet {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Planet {
            pos: Vec3 { x, y, z },
            vel: Vec3::default(),
        }
    }

    fn move_by_vel(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }

    fn energy(&self) -> i32 {
        self.pot_energy() * self.kin_energy()
    }

    fn pot_energy(&self) -> i32 {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn kin_energy(&self) -> i32 {
        self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs()
    }
}

#[derive(Default, Clone, Debug, Hash)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}
