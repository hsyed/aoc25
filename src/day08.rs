use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::str::FromStr;

use indexmap::IndexMap;

// Fixes
// * The IndexMap and sorting by x-coordinate were used earlier for a threshold-based approach.
//   This dependency is no longer needed, so the sorting step and IndexMap usage can be removed.


pub fn solve_problem_1(main_file: &str) -> std::io::Result<()> {
    let input = std::fs::read_to_string(main_file)?;

    let mut system: LighteningSystem = input.parse().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Parse error: {}", e),
        )
    })?;

    system.connect_junctions_n(1000);

    let mut sizes: Vec<u32> = system
        .unique_circuits()
        .map(|circuit| circuit.borrow().len() as u32)
        .collect();
    sizes.sort_by(|a, b| b.cmp(a)); // Sort descending
    let prod: u32 = sizes.iter().take(3).product();

    println!("problem 1: result = {}", prod);
    Ok(())
}

pub fn solve_problem_2(main_file: &str) -> std::io::Result<()> {
    let input = std::fs::read_to_string(main_file)?;

    let mut system: LighteningSystem = input.parse().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Parse error: {}", e),
        )
    })?;

    if let Some((a, b)) = system.connect_into_single_circuit() {
        println!("problem 2: result = {}", a.x as u64 * b.x as u64);
        Ok(())
    } else {
        panic!("boom");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Point3D { x, y, z }
    }

    fn distance_squared(&self, other: &Point3D) -> u64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        (dx * dx + dy * dy + dz * dz) as u64
    }
}

struct LighteningSystem {
    circuits: IndexMap<Point3D, Rc<RefCell<HashSet<Point3D>>>>,
}

impl FromStr for LighteningSystem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse all points
        let mut points: Vec<Point3D> = s
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let parts: Vec<i32> = line
                    .split(',')
                    .map(|p| p.trim().parse::<i32>())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| format!("Failed to parse number: {}", e))?;

                if parts.len() != 3 {
                    return Err(format!("Expected 3 coordinates, got {}", parts.len()));
                }

                Ok(Point3D::new(parts[0], parts[1], parts[2]))
            })
            .collect::<Result<Vec<_>, String>>()?;

        // Sort by x coordinate (ascending)
        points.sort_by_key(|p| p.x);

        // Populate circuits with empty sets
        let mut circuits = IndexMap::new();
        for point in points {
            circuits.insert(point, Rc::new(RefCell::new(HashSet::from([point]))));
        }
        Ok(LighteningSystem { circuits })
    }
}

impl LighteningSystem {
    fn unique_circuits(&self) -> impl Iterator<Item = Rc<RefCell<HashSet<Point3D>>>> + '_ {
        let mut seen = HashSet::new();
        self.circuits
            .values()
            .filter(move |circuit| seen.insert(Rc::as_ptr(*circuit)))
            .cloned()
    }

    fn merge_circuits(&mut self, point_a: Point3D, point_b: Point3D) -> bool {
        let set_a_rc = self.circuits.get(&point_a).unwrap().clone();
        let set_b_rc = self.circuits.get(&point_b).unwrap().clone();

        if Rc::ptr_eq(&set_a_rc, &set_b_rc) {
            return false; // Already in the same set - no merge happened
        }

        let mut set_a = set_a_rc.borrow_mut();
        let mut set_b = set_b_rc.borrow_mut();

        // Merge smaller set into larger set
        if set_a.len() < set_b.len() {
            for &point in set_a.iter() {
                set_b.insert(point);
                self.circuits.insert(point, set_b_rc.clone());
            }
        } else {
            for &point in set_b.iter() {
                set_a.insert(point);
                self.circuits.insert(point, set_a_rc.clone());
            }
        }

        true // Merge happened
    }

    fn connect_junctions_n(&mut self, wire_count: u32) -> u32 {
        // Pre-compute all pairs with distances
        let points: Vec<Point3D> = self.circuits.keys().copied().collect();
        let mut pairs: Vec<(Point3D, Point3D, u64)> = Vec::new();

        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let dist_sq = points[i].distance_squared(&points[j]);
                pairs.push((points[i], points[j], dist_sq));
            }
        }

        // Sort by distance
        pairs.sort_by_key(|(_, _, dist)| *dist);

        // Process first wire_count pairs (some may be no-ops if already connected)
        let mut n = 0;
        pairs
            .iter()
            .take(wire_count as usize)
            .for_each(|(a, b, _dist)| {
                let _ = self.merge_circuits(*a, *b);
                n += 1;
            });
        n
    }

    fn connect_into_single_circuit(&mut self) -> Option<(Point3D, Point3D)> {
        // Pre-compute all pairs with distances
        let points: Vec<Point3D> = self.circuits.keys().copied().collect();
        let mut pairs: Vec<(Point3D, Point3D, u64)> = Vec::new();
        let num_points = points.len();

        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let dist_sq = points[i].distance_squared(&points[j]);
                pairs.push((points[i], points[j], dist_sq));
            }
        }

        // Sort by distance
        pairs.sort_by_key(|(_, _, dist)| *dist);

        let mut merge_count = 0;
        for (a, b, _) in pairs.iter() {
            if self.merge_circuits(*a, *b) {
                merge_count += 1;
                if merge_count == num_points - 1 {
                    return Some((*a, *b));
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_sample_problem_1() {
        let mut system: LighteningSystem = TEST_INPUT.parse().unwrap();
        let connections_made = system.connect_junctions_n(10);
        assert_eq!(connections_made, 10);

        let unique_circuits = system.unique_circuits().count();
        assert_eq!(unique_circuits, 11);

        system.unique_circuits().for_each(|c| {
            println!("{}", c.borrow().len());
        });

        let mut sizes: Vec<u32> = system
            .unique_circuits()
            .map(|circuit| circuit.borrow().len() as u32)
            .collect();
        sizes.sort_by(|a, b| b.cmp(a)); // Sort descending
        let prod: u32 = sizes.iter().take(3).product();
        assert_eq!(prod, 40);
    }

    #[test]
    fn test_sample_problem_2() {
        let mut system: LighteningSystem = TEST_INPUT.parse().unwrap();

        if let Some((a, b)) = system.connect_into_single_circuit() {
            let result = a.x * b.x;
            assert_eq!(result, 25272);
        } else {
            panic!("boom");
        }
    }
}
