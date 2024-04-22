pub const GRAVITY_CONSTANT: f32 = 6.67430e-11f32; // division its temporal

pub fn calculate_force(mass1: f32, mass2: f32, distance: f32) -> f32 {
    if distance > 1. {
        return GRAVITY_CONSTANT * ((mass1 * mass2) / distance.powi(2));
    } else {
        return 0.;
    }
}
