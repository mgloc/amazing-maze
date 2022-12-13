use std::sync::{Arc, Mutex};

#[derive(PartialEq)]
pub enum Exploration {
    Explored,
    UnExplored,
}

pub enum Maze {
    Branch {
        label: String,
        left: Arc<Maze>,
        right: Arc<Maze>,
        status: Arc<Mutex<Exploration>>,
    },
    Leaf {
        label: String,
    },
}

pub fn sample_maze() -> Maze {
    let maze = Maze::Branch {
        label: "A".to_string(),
        left: Arc::new(Maze::Leaf {
            label: "B".to_string(),
        }),
        right: Arc::new(Maze::Branch {
            label: "C".to_string(),
            left: Arc::new(Maze::Leaf {
                label: "D".to_string(),
            }),
            right: Arc::new(Maze::Leaf {
                label: "E".to_string(),
            }),
            status: Arc::new(Mutex::new(Exploration::UnExplored)),
        }),
        status: Arc::new(Mutex::new(Exploration::UnExplored)),
    };
    maze
}
