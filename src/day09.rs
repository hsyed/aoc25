use std::str::FromStr;

pub fn solve_problem_1(main_file: &str) -> std::io::Result<()> {
    let input = std::fs::read_to_string(main_file)?;

    let system: FloorPlan = input.parse().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Parse error: {}", e),
        )
    })?;
    let largest_area = system.find_largest_area();
    println!("problem 1: result = {}", largest_area);
    Ok(())
}

pub fn solve_problem_2(main_file: &str) -> std::io::Result<()> {
    let input = std::fs::read_to_string(main_file)?;

    let system: FloorPlan = input.parse().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Parse error: {}", e),
        )
    })?;

    let largest_area = system.find_largest_valid_rectangle();
    println!("problem 2: result = {}", largest_area);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

struct FloorPlan {
    points: Vec<Point>,
}

impl FloorPlan {
    fn find_largest_area(&self) -> u64 {
        let mut largest_area = 0;
        for i in 0..self.points.len() {
            for j in i + 1..self.points.len() {
                let a = &self.points[i];
                let b = &self.points[j];
                let area = ((a.x - b.x).abs() as u64 + 1) * ((a.y - b.y).abs() as u64 + 1);
                if area > largest_area {
                    largest_area = area;
                }
            }
        }
        largest_area
    }

    fn find_largest_valid_rectangle(&self) -> u64 {
        let polygon = &self.points;
        let mut largest_area = 0;

        for i in 0..self.points.len() {
            for j in i + 1..self.points.len() {
                let a = &self.points[i];
                let b = &self.points[j];
                let area = ((a.x - b.x).abs() as u64 + 1) * ((a.y - b.y).abs() as u64 + 1);

                if area > largest_area && Self::is_rectangle_inside_polygon(a, b, polygon) {
                    largest_area = area;
                }
            }
        }
        largest_area
    }

    fn is_rectangle_inside_polygon(a: &Point, b: &Point, polygon: &[Point]) -> bool {
        let min_x = a.x.min(b.x);
        let max_x = a.x.max(b.x);
        let min_y = a.y.min(b.y);
        let max_y = a.y.max(b.y);

        // The other two corners of the rectangle
        let c = Point::new(min_x, max_y);
        let d = Point::new(max_x, min_y);

        // All 4 corners must be inside or on the polygon
        if !Self::point_inside_or_on_polygon(&c, polygon) {
            return false;
        }
        if !Self::point_inside_or_on_polygon(&d, polygon) {
            return false;
        }

        // No polygon edge can cross through the rectangle's interior
        !Self::polygon_edge_crosses_rect_interior(polygon, min_x, max_x, min_y, max_y)
    }

    fn point_inside_or_on_polygon(point: &Point, polygon: &[Point]) -> bool {
        let n = polygon.len();
        let (px, py) = (point.x, point.y);
        let mut inside = false;

        for i in 0..n {
            let j = (i + 1) % n;
            let (xi, yi) = (polygon[i].x, polygon[i].y);
            let (xj, yj) = (polygon[j].x, polygon[j].y);

            if xi == xj {
                // Vertical edge - check if point is on it
                if px == xi && py >= yi.min(yj) && py <= yi.max(yj) {
                    return true;
                }
                // Ray casting: vertical edge crosses horizontal ray
                if (yi > py) != (yj > py) && px < xi {
                    inside = !inside;
                }
            } else if yi == yj {
                // Horizontal edge - check if point is on it
                if py == yi && px >= xi.min(xj) && px <= xi.max(xj) {
                    return true;
                }
            }
        }

        inside
    }

    fn polygon_edge_crosses_rect_interior(
        polygon: &[Point],
        min_x: i32,
        max_x: i32,
        min_y: i32,
        max_y: i32,
    ) -> bool {
        let n = polygon.len();

        for i in 0..n {
            let j = (i + 1) % n;
            let (x1, y1) = (polygon[i].x, polygon[i].y);
            let (x2, y2) = (polygon[j].x, polygon[j].y);

            if x1 == x2 {
                // Vertical edge crosses interior if x is strictly inside
                if x1 > min_x && x1 < max_x {
                    let edge_min_y = y1.min(y2);
                    let edge_max_y = y1.max(y2);
                    if edge_min_y < max_y && edge_max_y > min_y {
                        return true;
                    }
                }
            } else if y1 == y2 {
                // Horizontal edge crosses interior if y is strictly inside
                if y1 > min_y && y1 < max_y {
                    let edge_min_x = x1.min(x2);
                    let edge_max_x = x1.max(x2);
                    if edge_min_x < max_x && edge_max_x > min_x {
                        return true;
                    }
                }
            }
        }

        false
    }
}

impl FromStr for FloorPlan {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<Point> = s
            .lines()
            .map(|line| {
                let parts: Vec<i32> = line
                    .split(',')
                    .map(|p| p.trim().parse::<i32>())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| format!("Failed to parse number: {}", e))?;

                if parts.len() != 2 {
                    return Err(format!("Expected 2 coordinates, got {}", parts.len()));
                }

                Ok(Point::new(parts[0], parts[1]))
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(FloorPlan { points })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_sample_problem_1() {
        let system: FloorPlan = TEST_INPUT.parse().unwrap();
        assert_eq!(system.find_largest_area(), 50);
    }

    #[test]
    fn test_sample_problem_2() {
        let system: FloorPlan = TEST_INPUT.parse().unwrap();
        assert_eq!(system.find_largest_valid_rectangle(), 24);
    }
}