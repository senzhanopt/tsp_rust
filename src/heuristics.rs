use core::f64;

use crate::models::{TspInstance, distance};

// Nearest Neighbor Heuristic
pub fn nearest_neighbor(tsp_instance: &TspInstance) -> Vec<usize> {
    // Initialize tour starting from the depot (index 0)
    let mut current_index = 0;
    let mut tour = vec![0];
    let mut unvisited: Vec<usize> = (1..=tsp_instance.cities.len()).collect();

    while !unvisited.is_empty() {
        // Find nearest unvisited city
        let mut nearest_distance = f64::MAX;
        let mut nearest_index = 0;
        for &city_index in &unvisited {
            let city = &tsp_instance.cities[city_index - 1];
            let current_city = if current_index == 0 {
                &tsp_instance.depot
            } else {
                &tsp_instance.cities[current_index - 1]
            };
            let dist = distance(current_city, city);
            if dist < nearest_distance {
                nearest_distance = dist;
                nearest_index = city_index;
            }
        }
        // Update tour and unvisited list
        tour.push(nearest_index);
        unvisited.retain(|&x| x != nearest_index);

        current_index = nearest_index;  
    }
    // Return to depot
    tour.push(0);
    tour
}

// 2-opt
pub fn two_opt(tour: &mut Vec<usize>, tsp_instance: &TspInstance) {
    let n = tour.len();
    if n <= 3 {
        return;
    }
    let mut improved = true;

    while improved {
        improved = false;
        for i in 0..(n - 3) {
            for j in (i + 2)..(n - 1) {
                let city_a = if tour[i] == 0 {
                    &tsp_instance.depot
                } else {
                    &tsp_instance.cities[tour[i] - 1]
                };
                let city_b = &tsp_instance.cities[tour[i+1] - 1];
                let city_c = &tsp_instance.cities[tour[j] - 1];
                let city_d = if tour[j+1] == 0 {
                    &tsp_instance.depot
                } else {
                    &tsp_instance.cities[tour[j+1] - 1]
                };

                let current_distance = distance(city_a, city_b) + distance(city_c, city_d);
                let new_distance = distance(city_a, city_c) + distance(city_b, city_d);

                if new_distance < current_distance {
                    tour[(i+1)..(j+1)].reverse();
                    improved = true;
                }
            }
        }
    }
}