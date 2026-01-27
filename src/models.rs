use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct City {
    pub id: usize,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TspInstance {
    pub name: String,
    pub depot: City,
    pub cities: Vec<City>,
}

// Distance function between two cities
pub fn distance(city1: &City, city2: &City) -> f64 {
    let dx = city1.x - city2.x;
    let dy = city1.y - city2.y;
    (dx * dx + dy * dy).sqrt()
}

// Tour distance function
// tour is a sequence of city indices, starting and ending at the depot (index 0)
pub fn tour_distance(tour: &[usize], tsp_instance: &TspInstance) -> f64 {
    let mut total_distance = 0.0;
    for i in 0..(tour.len()-1) {
        let city1 = if tour[i] == 0 {
            &tsp_instance.depot
        } else {
            &tsp_instance.cities[tour[i] - 1]
        };
        let city2 = if tour[i + 1] == 0 {
            &tsp_instance.depot
        } else {
            &tsp_instance.cities[tour[i + 1] - 1]
        };
        total_distance += distance(city1, city2);
    }
    total_distance
}
