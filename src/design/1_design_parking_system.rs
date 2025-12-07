/**
 * 1603. Design Parking System
 */
use std::collections::HashMap; // Fix typo: HahMap -> HashMap

struct ParkingSystem {
    parking_hashmap: HashMap<i32, i32>,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        let mut parking_hashmap = HashMap::new();
        parking_hashmap.insert(1, big);
        parking_hashmap.insert(2, medium);
        parking_hashmap.insert(3, small);

        ParkingSystem { parking_hashmap }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        // Changed to &mut self
        if let Some(availability) = self.parking_hashmap.get_mut(&car_type) {
            if *availability == 0 {
                return false;
            }
            *availability -= 1;
            true
        } else {
            false
        }
    }
}
