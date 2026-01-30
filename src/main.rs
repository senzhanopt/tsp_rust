mod heuristics;
mod models;

use heuristics::{nearest_neighbor, three_opt, two_opt};
use models::{TspInstance, build_distance_matrix, tour_distance};
use std::env;
use std::fs;

fn main() {
    // Read JSON input
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }
    let input_file = &args[1];
    let data = fs::read_to_string(input_file).expect("Unable to read file");
    let tsp_instance: TspInstance = serde_json::from_str(&data).expect("Unable to parse JSON");
    let distance_matrix = build_distance_matrix(&tsp_instance);
    let mut tour = nearest_neighbor(&tsp_instance, &distance_matrix);
    two_opt(&mut tour, &tsp_instance, &distance_matrix);
    three_opt(&mut tour, &tsp_instance, &distance_matrix);
    let total_distance = tour_distance(&tour, &tsp_instance);
    println!("Tour: {:?}", tour);
    println!("Total distance: {:.2}", total_distance);
    // Save output to file
    let output_file = &args[2];
    let output_data = serde_json::to_string_pretty(&tour).expect("Unable to serialize tour");
    fs::write(output_file, output_data).expect("Unable to write file");
    println!("Tour saved to {}", output_file);
}
