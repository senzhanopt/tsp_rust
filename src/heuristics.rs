use core::f64;

use crate::models::TspInstance;

const EPSILON: f64 = 1e-8;

// Nearest Neighbor Heuristic
pub fn nearest_neighbor(tsp_instance: &TspInstance, distance_matrix: &Vec<Vec<f64>>) -> Vec<usize> {
    // Initialize tour starting from the depot (index 0)
    let mut current_index = 0;
    let mut tour = vec![0];
    let mut unvisited: Vec<usize> = (1..=tsp_instance.cities.len()).collect();

    while !unvisited.is_empty() {
        // Find nearest unvisited city
        let mut nearest_distance = f64::MAX;
        let mut nearest_index = 0;
        for &city_index in &unvisited {
            let dist = distance_matrix[current_index][city_index];
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
pub fn two_opt(
    tour: &mut Vec<usize>,
    _tsp_instance: &TspInstance,
    distance_matrix: &Vec<Vec<f64>>,
) {
    let n = tour.len();
    if n <= 3 {
        return;
    }
    let mut improved = true;

    while improved {
        improved = false;
        'outer: for i in 0..(n - 3) {
            for j in (i + 2)..(n - 1) {
                let current_distance =
                    distance_matrix[tour[i]][tour[i + 1]] + distance_matrix[tour[j]][tour[j + 1]];
                let new_distance =
                    distance_matrix[tour[i]][tour[j]] + distance_matrix[tour[i + 1]][tour[j + 1]];

                if new_distance < current_distance - EPSILON {
                    tour[(i + 1)..(j + 1)].reverse();
                    improved = true;
                    break 'outer;
                }
            }
        }
    }
}

pub fn three_opt(
    tour: &mut Vec<usize>,
    _tsp_instance: &TspInstance,
    distance_matrix: &Vec<Vec<f64>>,
) {
    let n = tour.len();
    if n <= 5 {
        return;
    }
    let mut improved = true;

    while improved {
        improved = false;
        'outer: for i in 0..(n - 5) {
            for j in (i + 2)..(n - 3) {
                for k in (j + 2)..(n - 1) {
                    let current_distance = distance_matrix[tour[i]][tour[i + 1]]
                        + distance_matrix[tour[j]][tour[j + 1]]
                        + distance_matrix[tour[k]][tour[k + 1]];

                    let new_distance_0 = distance_matrix[tour[i]][tour[j]]
                        + distance_matrix[tour[i + 1]][tour[j + 1]]
                        + distance_matrix[tour[k]][tour[k + 1]];

                    let new_distance_1 = distance_matrix[tour[i]][tour[i + 1]]
                        + distance_matrix[tour[j]][tour[k]]
                        + distance_matrix[tour[j + 1]][tour[k + 1]];

                    let new_distance_2 = distance_matrix[tour[i]][tour[j]]
                        + distance_matrix[tour[i + 1]][tour[k]]
                        + distance_matrix[tour[j + 1]][tour[k + 1]];

                    let new_distance_3 = distance_matrix[tour[i]][tour[j + 1]]
                        + distance_matrix[tour[k]][tour[i + 1]]
                        + distance_matrix[tour[j]][tour[k + 1]];

                    let new_distance_4 = distance_matrix[tour[i]][tour[j + 1]]
                        + distance_matrix[tour[k]][tour[j]]
                        + distance_matrix[tour[i + 1]][tour[k + 1]];

                    let new_distance_5 = distance_matrix[tour[i]][tour[k]]
                        + distance_matrix[tour[j + 1]][tour[i + 1]]
                        + distance_matrix[tour[j]][tour[k + 1]];

                    let new_distance_6 = distance_matrix[tour[i]][tour[k]]
                        + distance_matrix[tour[j + 1]][tour[j]]
                        + distance_matrix[tour[i + 1]][tour[k + 1]];

                    // Find the best new distance
                    let new_distances = [
                        new_distance_0,
                        new_distance_1,
                        new_distance_2,
                        new_distance_3,
                        new_distance_4,
                        new_distance_5,
                        new_distance_6,
                    ];
                    let (best_index, &best_distance) = new_distances
                        .iter()
                        .enumerate()
                        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                        .unwrap();
                    if best_distance < current_distance - EPSILON {
                        match best_index {
                            0 => {
                                // a->c->b->d->e->f
                                tour[(i + 1)..(j + 1)].reverse();
                            }
                            1 => {
                                // a->b->c->e->d->f
                                tour[(j + 1)..(k + 1)].reverse();
                            }
                            2 => {
                                // a->c->b->e->d->f
                                tour[(i + 1)..(j + 1)].reverse();
                                tour[(j + 1)..(k + 1)].reverse();
                            }
                            3 => {
                                // a->d->e->b->c->f
                                tour[(i + 1)..(k + 1)].reverse();
                                tour[(i + 1)..(i + 1 + (k - j))].reverse();
                                tour[(i + 1 + (k - j))..(k + 1)].reverse();
                            }
                            4 => {
                                // a->d->e->c->b->f
                                tour[(i + 1)..(k + 1)].reverse();
                                tour[(i + 1)..(i + 1 + (k - j))].reverse();
                            }
                            5 => {
                                // a->e->d->b->c->f
                                tour[(i + 1)..(k + 1)].reverse();
                                tour[(i + 1 + (k - j))..(k + 1)].reverse();
                            }
                            6 => {
                                // a->e->d->c->b->f
                                tour[(i + 1)..(k + 1)].reverse();
                            }
                            _ => {}
                        }
                        improved = true;
                        break 'outer;
                    }
                }
            }
        }
    }
}
