fn from_amsterdam_to_paris<T: Vehicle>(vehicle: T) {
    //                      ^-------- Trait bound
    while (location_of(vehicle) != "Paris") {
        vehicle.move()
    }
}
