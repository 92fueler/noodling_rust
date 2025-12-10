/**
 * 1603. Design Parking System
 */
use std::collections::HashMap;

#[repr(i32)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum CarType {
    Small = 1,
    Medium = 2,
    Big = 3,
}

#[derive(Debug)]
struct ParkingSystem {
    parking_hashmap: HashMap<CarType, i32>,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        let mut hm = HashMap::new();
        hm.insert(CarType::Big, big);
        hm.insert(CarType::Medium, medium);
        hm.insert(CarType::Small, small);

        ParkingSystem {
            parking_hashmap: hm,
        }
    }

    fn add_car(&mut self, car_type: CarType) -> bool {
        // car_type shouldn't use i32 any more if using custom defined enum type

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

fn main() {
    let mut parking = ParkingSystem::new(1, 2, 3);
    parking.add_car(CarType::Small);
    parking.add_car(CarType::Small);

    parking.add_car(CarType::Big);
    parking.add_car(CarType::Big);

    parking.add_car(CarType::Medium);

    println!("{:?}", parking);
}
