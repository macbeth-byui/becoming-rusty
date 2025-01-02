use std::cmp::Ordering;

const TOLERANCE: f64 = 0.001;

#[derive(Debug, PartialEq)]
pub enum Angle {
    Convex,
    Concave,
    Colinear,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x : f64,
    pub y : f64,
}

impl Eq for Point {

}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.x, other.y).partial_cmp(&(self.x, self.y)).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= TOLERANCE && 
        (self.y - other.y).abs() <= TOLERANCE 
    }
}

fn orientation(a : &Point, b : &Point, c : &Point) -> Angle {
    let value = (b.x - a.x) * (c.y - b.y) - (b.y - a.y) * (c.x - b.x);
    match value {
        value if value > 0.0 => Angle::Convex,
        value if value < 0.0 => Angle::Concave,
        _ => Angle::Colinear
    }
}

fn get_angle(a : &Point, b : &Point) -> f64 {
    (b.y - a.y).atan2(b.x - a.x)
}

fn get_dist(a : &Point, b : &Point) -> f64 {
    ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt()
}

pub fn gen_hull(points : &[Point]) -> Option<Vec<Point>> {

    if points.len() < 3 {
        return None;
    }

    let anchor = points
        .iter().min_by(|a, b| {
            let a = (a.y, a.x);
            let b = (b.y, b.x);
            a.partial_cmp(&b).unwrap_or(Ordering::Equal)
        }).unwrap();
    
    let mut sorted = points.to_vec();
    sorted.sort_by(|a, b| {
        let a = (get_angle(anchor, a), get_dist(anchor, a));
        let b = (get_angle(anchor, b), get_dist(anchor, b));
        a.partial_cmp(&b).unwrap_or(Ordering::Equal)
    });
    sorted.push(anchor.clone());

    let mut hull : Vec<Point> = vec![sorted[0].clone()];

    for p in sorted.iter().skip(1) {
        loop {
            match hull.len() {
                0 => return None,
                1 => {
                    hull.push(p.clone());
                    break;
                }
                _ => {
                    let a = hull.get(hull.len()-2).unwrap();
                    let b = hull.last().unwrap();
                    match orientation(a, b, p) {
                        Angle::Convex => {
                            hull.push(p.clone());
                            break;
                        }
                        _ => {
                            hull.pop();
                        }
                    }
                }
            }
        }
    }
    if hull.len() < 3 { None } else { Some(hull) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_orientation_convex() {
        let a = Point {x:0.0, y:0.0};
        let b = Point {x:4.0, y:0.0};
        let c = Point {x:3.0, y:1.0};
        let result = orientation(&a, &b, &c);
        assert_eq!(result, Angle::Convex)
    }

    #[test]
    fn test2_orientation_concave() {
        let a = Point {x:4.0, y:0.0};
        let b = Point {x:3.0, y:1.0};
        let c = Point {x:8.0, y:8.0};
        let result = orientation(&a, &b, &c);
        assert_eq!(result, Angle::Concave)
    }

    #[test]
    fn test3_orientation_colinear() {
        let a = Point {x:0.0, y:0.0};
        let b = Point {x:1.0, y:1.0};
        let c = Point {x:8.0, y:8.0};
        let result = orientation(&a, &b, &c);
        assert_eq!(result, Angle::Colinear)
    }

    #[test]
    fn test4_get_angle() {
        let a = Point {x:1.0, y:2.0};
        let b = Point {x:4.0, y:7.0};
        let result = get_angle(&a, &b);
        assert!((result - 1.030).abs() <= TOLERANCE)
    }

    #[test]
    fn test5_get_dist() {
        let a = Point {x:1.0, y:2.0};
        let b = Point {x:4.0, y:7.0};
        let result = get_dist(&a, &b);
        assert!((result - 5.831).abs() <= TOLERANCE)
    }

    #[test]
    fn test6_gen_hull() {
        let points = vec![
            Point {x:0.0, y:0.0},
            Point {x:4.0, y:0.0},
            Point {x:3.0, y:1.0},
            Point {x:1.0, y:1.0},
            Point {x:8.0, y:8.0},
            Point {x:3.0, y:6.0},
            Point {x:1.0, y:4.0},
            Point {x:1.0, y:3.0},
            Point {x:0.0, y:4.0},
            Point {x:0.0, y:2.0},
            Point {x:5.5, y:7.0},
        ];
        let hull = gen_hull(&points);
        assert!(hull.is_some());
        assert_eq!(hull.unwrap(), vec![
            Point {x:0.0, y:0.0},
            Point {x:4.0, y:0.0},
            Point {x:8.0, y:8.0},
            Point {x:3.0, y:6.0},
            Point {x:0.0, y:4.0},
            Point {x:0.0, y:0.0},
        ]);
    }

    #[test]
    fn test7_gen_hull_too_small() {
        let points = vec![
            Point {x:0.0, y:0.0},
            Point {x:4.0, y:0.0},
        ];
        let hull = gen_hull(&points);
        assert!(hull.is_none());
    }

    #[test]
    fn test8_gen_hull_too_small_1() {
        let points = vec![
            Point {x:0.0, y:0.0},
        ];
        let hull = gen_hull(&points);
        assert!(hull.is_none());
    }

    #[test]
    fn test9_gen_hull_too_small_0() {
        let points = vec![];
        let hull = gen_hull(&points);
        assert!(hull.is_none());
    }

    #[test]
    fn test10_gen_hull_all_colinear() {
        let points = vec![
            Point {x:0.0, y:0.0},
            Point {x:1.0, y:1.0},
            Point {x:2.0, y:2.0},
            Point {x:3.0, y:3.0},
            Point {x:4.0, y:4.0},
        ];
        let hull = gen_hull(&points);
        assert!(hull.is_none());
    }

    #[test]
    fn test11_gen_hull_colinear_at_start() {
        let points = vec![
            Point {x:0.0, y:0.0},
            Point {x:1.0, y:0.0},
            Point {x:2.0, y:0.0},
            Point {x:3.0, y:0.0},
            Point {x:4.0, y:0.0},
            Point {x:3.0, y:1.0},
            Point {x:1.0, y:1.0},
            Point {x:8.0, y:8.0},
            Point {x:3.0, y:6.0},
            Point {x:1.0, y:4.0},
            Point {x:1.0, y:3.0},
            Point {x:0.0, y:4.0},
            Point {x:0.0, y:2.0},
            Point {x:5.5, y:7.0},
        ];
        let hull = gen_hull(&points);
        assert!(hull.is_some());
        assert_eq!(hull.unwrap(), vec![
            Point {x:0.0, y:0.0},
            Point {x:4.0, y:0.0},
            Point {x:8.0, y:8.0},
            Point {x:3.0, y:6.0},
            Point {x:0.0, y:4.0},
            Point {x:0.0, y:0.0},
        ]);
    }


}