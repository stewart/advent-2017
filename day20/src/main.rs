use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct Particle {
    position: (isize, isize, isize),
    acceleration: (isize, isize, isize),
    velocity: (isize, isize, isize),
}

impl Particle {
    fn absolute_acceleration(&self) -> usize {
        let (ax, ay, az) = self.acceleration;
        (ax.abs() + ay.abs() + az.abs()) as usize
    }
}

impl FromStr for Particle {
    type Err = String;

    fn from_str(input: &str) -> Result<Particle, Self::Err> {
        let err = format!("Unable to parse: {}", input);
        let mut particle = Particle {
            position: (0, 0, 0),
            acceleration: (0, 0, 0),
            velocity: (0, 0, 0),
        };

        for section in input.split(", ") {
            let mut section = section.chars();
            let attribute = section.next().expect("Attribute");

            // discard '=', '<'
            section.next();
            section.next();

            let x: String = { section.by_ref().take_while(|&x| x != ',' && x != '>').collect() };
            let y: String = { section.by_ref().take_while(|&x| x != ',' && x != '>').collect() };
            let z: String = section.take_while(|&x| x != ',' && x != '>').collect();

            let x: isize = x.parse().expect("X");
            let y: isize = y.parse().expect("Y");
            let z: isize = z.parse().expect("Z");

            match attribute {
                'p' => { particle.position = (x, y, z) }
                'a' => { particle.acceleration = (x, y, z) }
                'v' => { particle.velocity = (x, y, z) }
                _ => {}
            };
        }

        Ok(particle)
    }
}

fn main() {
    let particles: Vec<Particle> = include_str!("input").
        trim().
        lines().
        map(|line| line.parse().unwrap()).
        collect();

    let (idx, _) = particles.
        iter().
        enumerate().
        min_by_key(|&(_, ref p)| p.absolute_acceleration()).
        unwrap();

    println!("1 -> {}", idx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_from_str() {
        assert_eq!(
            "p=<-13053,-6894,1942>, v=<14,39,-11>, a=<16,7,-2>".parse(),
            Ok(Particle {
                position: (-13053, -6894, 1942),
                acceleration: (16, 7, -2),
                velocity: (14, 39, -11)
            })
        )
    }
}
