pub fn is_within_angle_range(angle_1: f32, angle_2: f32, angle_amount: f32) -> bool {
    let diff = ((angle_1 - angle_2 + 180.0) % 360.0 - 180.0).abs();
    diff <= angle_amount
}