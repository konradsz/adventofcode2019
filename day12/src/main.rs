use std::cmp::Ordering;

struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}

struct Position {
    x: i32,
    y: i32,
    z: i32,
}

struct Moon {
    velocity: Velocity,
    position: Position,
}

impl Moon {
    fn new(position: Position) -> Self {
        Moon {
            velocity: Velocity { x: 0, y: 0, z: 0 },
            position,
        }
    }
}

fn apply_gravity(moons: &mut Vec<Moon>, index1: usize, index2: usize) {
    match moons[index1].position.x.cmp(&moons[index2].position.x) {
        Ordering::Greater => {
            moons[index1].velocity.x -= 1;
            moons[index2].velocity.x += 1;
        }
        Ordering::Less => {
            moons[index1].velocity.x += 1;
            moons[index2].velocity.x -= 1;
        }
        Ordering::Equal => (),
    };

    match moons[index1].position.y.cmp(&moons[index2].position.y) {
        Ordering::Greater => {
            moons[index1].velocity.y -= 1;
            moons[index2].velocity.y += 1;
        }
        Ordering::Less => {
            moons[index1].velocity.y += 1;
            moons[index2].velocity.y -= 1;
        }
        Ordering::Equal => (),
    };

    match moons[index1].position.z.cmp(&moons[index2].position.z) {
        Ordering::Greater => {
            moons[index1].velocity.z -= 1;
            moons[index2].velocity.z += 1;
        }
        Ordering::Less => {
            moons[index1].velocity.z += 1;
            moons[index2].velocity.z -= 1;
        }
        Ordering::Equal => (),
    };
}

fn apply_gravities(moons: &mut Vec<Moon>) {
    apply_gravity(moons, 0, 1);
    apply_gravity(moons, 0, 2);
    apply_gravity(moons, 0, 3);
    apply_gravity(moons, 1, 2);
    apply_gravity(moons, 1, 3);
    apply_gravity(moons, 2, 3);
}

fn apply_velocity(m1: &mut Moon) {
    m1.position.x += m1.velocity.x;
    m1.position.y += m1.velocity.y;
    m1.position.z += m1.velocity.z;
}

fn apply_velocities(moons: &mut Vec<Moon>) {
    moons.iter_mut().for_each(|moon| apply_velocity(moon));
}

fn calculate_potential_energy(m: &Moon) -> i32 {
    m.position.x.abs() + m.position.y.abs() + m.position.z.abs()
}

fn calculate_kinetic_energy(m: &Moon) -> i32 {
    m.velocity.x.abs() + m.velocity.y.abs() + m.velocity.z.abs()
}

fn calculate_total_energy(moons: &[Moon]) -> i32 {
    moons
        .iter()
        .map(|moon| calculate_potential_energy(moon) * calculate_kinetic_energy(moon))
        .sum()
}

fn part_1() {
    let mut moons = vec![
        Moon::new(Position {
            x: 16,
            y: -8,
            z: 13,
        }),
        Moon::new(Position { x: 4, y: 10, z: 10 }),
        Moon::new(Position { x: 17, y: -5, z: 6 }),
        Moon::new(Position { x: 13, y: -3, z: 0 }),
    ];

    for _ in 0..1_000 {
        apply_gravities(&mut moons);
        apply_velocities(&mut moons);
    }

    assert_eq!(7_687, calculate_total_energy(&moons))
}

fn main() {
    part_1();
    // PART 2:
    // x, y, z coordinates change independently from each other
    // x comes back to its original value every 113028 steps
    // y comes back to its original value every 231614 steps
    // z comes back to its original value every 102356 steps
    // LCM(113028, 231614, 102356) = 334945516288044
}
