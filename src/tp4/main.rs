mod models;
use models::{sample_maze, Exploration, Maze};

use std::sync::{Arc, Mutex};
use std::thread;

impl Maze {
    pub fn explore(&self, work: Arc<Mutex<Vec<Arc<Maze>>>>, mut trace: Vec<String>) -> Vec<String> {
        match self {
            Maze::Branch {
                label,
                left,
                right,
                status,
            } => {
                let current_arc_status = status.clone();
                let mut current_status = current_arc_status.lock().unwrap();
                match *current_status {
                    Exploration::UnExplored => {
                        // On marque le noeud comme exploré
                        *current_status = Exploration::Explored;
                        // On push notre action sur la trace
                        trace.push(label.to_string());
                        // On push la droite dans le worker

                        work.lock().unwrap().push(Arc::clone(&right));
                        // On continu le travail à gauche

                        return left.explore(work, trace);
                    }
                    Exploration::Explored => return trace,
                }
            }
            Maze::Leaf { label } => {
                trace.push(label.to_string());
                return trace;
            }
        }
    }
}

fn get_finished_thread(
    threads: &mut Vec<thread::JoinHandle<Vec<String>>>,
) -> Option<thread::JoinHandle<Vec<String>>> {
    for (i, handle) in threads.iter().enumerate() {
        if handle.is_finished() {
            let h = threads.remove(i);
            return Some(h);
        }
    }
    None
}

pub fn main() {
    let maze: Arc<Maze> = Arc::new(sample_maze());
    let work = Arc::new(Mutex::new(vec![Arc::clone(&maze)]));

    // Création du tableau qui stock les threads
    let mut threads = vec![];

    loop {
        while let Some(node) = work.clone().lock().unwrap().pop() {
            let node_copy = Arc::clone(&node);
            let trace: Vec<String> = vec![];
            let work_copy = work.clone();
            let handle = thread::spawn(move || node_copy.explore(work_copy, trace));
            threads.push(handle);
        }
        if !threads.is_empty() {
            let handle = get_finished_thread(&mut threads);
            match handle {
                Some(h) => {
                    let id = h.thread().id();
                    let trace = h.join().unwrap();
                    println!("Trace {:?}, ({:?})", trace, id);
                }
                None => {}
            }
        } else {
            break;
        }
    }
}
