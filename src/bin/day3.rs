use std::fmt;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    x: i32,
    y: i32,
    n: u32,
    stress_sum: u64
}

impl Node {
    fn address(&self) -> String {
        return format!("{},{}", self.x, self.y);
    }
}

struct Grid {
    nodes: Vec<Rc<Node>>,
    //by_address: HashMap<String, Rc<Node>>,
    min: (i32, i32),
    max: (i32, i32),
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({:?}, {:?})", self.min, self.max);
    }
}

static DIRECTIONS: &'static [(i32, i32)] = &[
    ( 1,  0), // right
    ( 0,  1), // up
    (-1,  0), // left
    ( 0, -1), // down
];

fn main() {
    part1();
}

fn part1() {
    let n = 277678;
    let grid = build_grid(n, false);
    let ln = grid.nodes.last().unwrap();
    println!("For n = {}, distance = {}", n, ln.x.abs() + ln.y.abs());

    let grid = build_grid(n, true);
    let ln = grid.nodes.last().unwrap();
    println!("Stress For n = {}, stress = {}", n, ln.stress_sum);
}

fn build_grid(n: u32, stress: bool) -> Grid {
    let mut nodes = vec![];
    let mut by_address = HashMap::new();
    let mut at: (i32, i32) = (0, 0);
    let mut min: (i32, i32) = (0, 0);
    let mut max: (i32, i32) = (0, 0);
    let mut direction_index: usize = 0;


    for i in 1..(n+1) {

        let node = Node {
            x: at.0,
            y: at.1,
            n: i,
            stress_sum: if stress {calc_stress(at, &by_address)} else {0}
        };
        let rc_node = Rc::new(node);
        nodes.push(rc_node.clone());
        by_address.insert(rc_node.address(), rc_node.clone());

        if rc_node.stress_sum > n as u64 {
            break;
        }


        let direction = DIRECTIONS[direction_index];
        at.0 += direction.0;
        at.1 += direction.1;

        if (direction.0 < 0 && at.0 < min.0) ||
           (direction.0 > 0 && at.0 > max.0) ||
           (direction.1 < 0 && at.1 < min.1) ||
           (direction.1 > 0 && at.1 > max.1) {
            direction_index += 1;
            if direction_index >= DIRECTIONS.len() {
                direction_index = 0;
            }
        }

        min.0 = std::cmp::min(min.0, at.0);
        min.1 = std::cmp::min(min.1, at.1);
        max.0 = std::cmp::max(max.0, at.0);
        max.1 = std::cmp::max(max.1, at.1);
    }

    return Grid {nodes, /*by_address,*/ min, max};
}

fn calc_stress(at: (i32, i32), by_address: &HashMap<String, Rc<Node>>) -> u64 {
    // println!("For {:?}", at);
    let mut sum = 0u64;
    for x in -1..2 {
        for y in -1..2 {
            let n = Node {
                x: at.0 + x,
                y: at.1 + y,
                n: 0,
                stress_sum: 0
            };
            let a = n.address();
            let neighbor = by_address.get(&a);
            // println!("\tChecking {} got {:?}", a, neighbor);
            match neighbor {
                Some(n) => sum += n.stress_sum,
                None => ()
            }
        }
    }
    return if sum > 0  {sum} else {1};
}
