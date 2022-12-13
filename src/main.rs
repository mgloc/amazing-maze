#[path = "./tp3/main.rs"]
mod tp3;

#[path = "./tp4/main.rs"]
mod tp4;

fn main() {
    println!("---- TP3: Labyrinthe sans threads");
    tp3::main();
    println!("--------------- Fin TP3 ---------------");
    println!("---- TP4: Labyrinthe avec threads");
    tp4::main();
    println!("----------------- Fin TP4 ---------------");
}
