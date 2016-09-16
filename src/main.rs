/// Conway's Game of Life

extern crate rand;

use rand::Rng;
use std::time::Duration;
use std::thread;


const F: usize = 54;
const C: usize = 70;

const DELAY_MS : u64 = 100;

fn main() {

    let mut canvas : [[bool; C]; F] = [[false; C]; F];

    let mut vecinos : [[u8; C]; F] = [[0; C]; F];

    // Estado inicial aleatorio:
    for i in 0..F {
        for j in 0..C {
            canvas[i][j] = rand::thread_rng().gen_range(0, 2) == 0; 
        }
    }

    loop {


        for i in 0..F {
            if i == 0 {
                for j in 0..C {
                    print!("-");
                }
                println!("");
            }
            for j in 0..C {
                // Display:
                if canvas[i][j] {print!("*")} else {print!(" ")};
                // Cálculo de vecinos:
                vecinos[i][j] = 0;

                if i > 0 {
                    // Los 3 de arriba
                    if canvas[i - 1][j] {vecinos[i][j] += 1};
                    if j > 0 { if canvas[i - 1][j - 1] {vecinos[i][j] += 1}; }
                    if j < C - 1 { if canvas[i - 1][j + 1] {vecinos[i][j] += 1}; }
                }

                if j > 0 {
                    // Los 2 de izquierda y abajo
                    if canvas[i][j - 1] {vecinos[i][j] += 1};
                    if i < F - 1 { if canvas[i + 1][j - 1] {vecinos[i][j] += 1}; }
                }
                
                if i < F - 1 {
                    // Los dos de abajo y derecha
                    if canvas[i + 1][j] {vecinos[i][j] += 1};
                    if j < C - 1 { if canvas[i + 1][j + 1] {vecinos[i][j] += 1}; }
                }

                if j < C - 1{
                    // El de adelante
                    if canvas[i][j + 1] {vecinos[i][j] += 1};
                }

                assert!(vecinos[i][j] <= 8, "Vecinos mal calculados");
            }
            print!("\n");
            if i == F - 1 {
                for j in 0..C {
                    print!("-");
                }
                println!("");
            }
        }

        // Evolución:
        for i in 0..F {
            for j in 0..C {
                // Soledad:
                if vecinos[i][j] < 2 {canvas[i][j] = false};
                // Sobrepoblación:
                if vecinos[i][j] > 3 {canvas[i][j] = false};
                // Nacimiento:
                if vecinos[i][j] == 3 {canvas[i][j] = true};
            }
        }

        thread::sleep(Duration::from_millis(DELAY_MS))
    }
}
