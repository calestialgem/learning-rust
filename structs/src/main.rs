struct Vector(f32, f32);

fn main() {
    let velocity = Vector(0.0, 0.0);
    let acceleration = Vector(0.0, -9.81);
    println!(
        "The vertical velocity would be {} m/s after one 1 second at {} m down from the initial position.",
        velocity.1 + acceleration.1, -acceleration.1/2.0
    );
}
