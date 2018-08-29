fn from_amsterdam_to_paris<T>(vehicle: T) {
    while (! vehicle.at_paris()) {
        vehicle.move()
    }
}
