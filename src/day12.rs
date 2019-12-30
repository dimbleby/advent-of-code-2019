use crate::util::lcm;

#[derive(Copy, Clone, PartialEq, Eq)]
struct Vector3D(i64, i64, i64);

#[derive(Copy, Clone, PartialEq, Eq)]
struct Moon {
    position: Vector3D,
    velocity: Vector3D,
}

impl Moon {
    fn new(position: Vector3D) -> Self {
        let velocity = Vector3D(0, 0, 0);
        Self { position, velocity }
    }

    fn potential_energy(&self) -> i64 {
        self.position.0.abs() + self.position.1.abs() + self.position.2.abs()
    }

    fn kinetic_energy(&self) -> i64 {
        self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs()
    }

    fn energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[derive(Clone, PartialEq, Eq)]
struct System {
    moons: Vec<Moon>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Axis {
    X,
    Y,
    Z,
}

impl System {
    fn new(moons: Vec<Moon>) -> Self {
        Self { moons }
    }

    fn step(&mut self) {
        // Calculate and apply velocity changes.
        let positions = self.moons.iter().map(|m| m.position).collect::<Vec<_>>();
        for moon in &mut self.moons {
            let delta = Self::get_gravity_delta(&moon.position, &positions);
            moon.velocity.0 += delta.0;
            moon.velocity.1 += delta.1;
            moon.velocity.2 += delta.2;
        }

        // Update positions.
        for moon in &mut self.moons {
            moon.position.0 += moon.velocity.0;
            moon.position.1 += moon.velocity.1;
            moon.position.2 += moon.velocity.2;
        }
    }

    fn get_gravity_delta(position: &Vector3D, others: &[Vector3D]) -> Vector3D {
        let mut delta = Vector3D(0, 0, 0);
        for other in others {
            delta.0 += (other.0 - position.0).signum();
            delta.1 += (other.1 - position.1).signum();
            delta.2 += (other.2 - position.2).signum();
        }
        delta
    }

    fn energy(&self) -> i64 {
        self.moons.iter().map(|m| m.energy()).sum()
    }

    fn keep_axis(&mut self, axis: Axis) {
        match axis {
            Axis::X => {
                for moon in &mut self.moons {
                    moon.position.1 = 0;
                    moon.position.2 = 0;
                }
            }
            Axis::Y => {
                for moon in &mut self.moons {
                    moon.position.0 = 0;
                    moon.position.2 = 0;
                }
            }
            Axis::Z => {
                for moon in &mut self.moons {
                    moon.position.0 = 0;
                    moon.position.1 = 0;
                }
            }
        }
    }

    fn find_cycle_length(&self) -> usize {
        let mut system = self.clone();
        let mut counter = 0;
        loop {
            system.step();
            counter += 1;
            if &system == self {
                break;
            };
        }
        counter
    }
}

pub(crate) fn day12() {
    // <x=-7, y=17, z=-11>
    // <x=9, y=12, z=5>
    // <x=-9, y=0, z=-4>
    // <x=4, y=6, z=0>
    let io = Moon::new(Vector3D(-7, 17, -11));
    let europa = Moon::new(Vector3D(9, 12, 5));
    let ganymede = Moon::new(Vector3D(-9, 0, -4));
    let callista = Moon::new(Vector3D(4, 6, 0));
    let system = System::new(vec![io, europa, ganymede, callista]);

    // Part one.
    let mut part_one = system.clone();
    for _ in 0..1000 {
        part_one.step();
    }
    let answer = part_one.energy();
    println!("Part one answer is: {}", answer);

    // Part two.  Emulate each axis independently, then recombine.
    let mut x_only = system.clone();
    x_only.keep_axis(Axis::X);
    let x_cycle = x_only.find_cycle_length();

    let mut y_only = system.clone();
    y_only.keep_axis(Axis::Y);
    let y_cycle = y_only.find_cycle_length();

    let mut z_only = system;
    z_only.keep_axis(Axis::Z);
    let z_cycle = z_only.find_cycle_length();

    let answer = lcm(lcm(x_cycle, y_cycle), z_cycle);
    println!("Part two answer is: {}", answer);
}
