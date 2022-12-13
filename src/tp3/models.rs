use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
pub enum Exploration {
    Explored,
    UnExplored,
}

pub enum Maze {
    Branch {
        label: String,
        left: Rc<Maze>,
        right: Rc<Maze>,
        status: RefCell<Exploration>,
    },
    Leaf {
        label: String,
    },
}

impl Maze {
    pub fn explore(&self, work: &mut Vec<Rc<Maze>>, trace: &mut Vec<String>) {
        match self {
            Maze::Branch {
                label,
                left,
                right,
                status,
            } => {
                if *status == RefCell::new(Exploration::UnExplored) {
                    work.push(Rc::clone(&right));
                    work.push(Rc::clone(&left));
                    trace.push(label.to_string());
                    status.replace(Exploration::Explored);
                }
            }
            Maze::Leaf { label } => {
                trace.push(label.to_string());
            }
        }
    }
}
