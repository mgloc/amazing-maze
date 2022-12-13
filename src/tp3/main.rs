mod models;
use models::{Exploration, Maze};

use std::cell::RefCell;
use std::rc::Rc;

fn sample_maze() -> Maze {
    let maze = Maze::Branch {
        label: "A".to_string(),
        left: Rc::new(Maze::Leaf {
            label: "B".to_string(),
        }),
        right: Rc::new(Maze::Branch {
            label: "C".to_string(),
            left: Rc::new(Maze::Leaf {
                label: "D".to_string(),
            }),
            right: Rc::new(Maze::Leaf {
                label: "E".to_string(),
            }),
            status: RefCell::new(Exploration::UnExplored),
        }),
        status: RefCell::new(Exploration::UnExplored),
    };
    maze
}

pub fn main() {
    let maze: Rc<Maze> = Rc::new(sample_maze());
    let mut work: Vec<Rc<Maze>> = vec![Rc::clone(&maze)];
    let mut trace: Vec<String> = vec![];
    while work.len() != 0 {
        let node = work.pop().expect("unexpected");
        node.explore(&mut work, &mut trace);
        println!("trace so far: {:?}", trace);
    }
}
