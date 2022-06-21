use crate::Exercise;

pub fn generate(exercise: &Exercise, expected_one_rep_max: f64) {
    let rep_coefficients = [
        1.0, 0.97, 0.94, 0.92, 0.89, 0.86, 0.83, 0.81, 0.78, 0.75, 0.73, 0.71, 0.7, 0.68, 0.67,
        0.65, 0.64, 0.63, 0.61, 0.6, 0.59, 0.58, 0.57, 0.56, 0.55, 0.54, 0.53, 0.52, 0.51, 0.5,
    ];

    //at best we tolerate 75% lower than the ceiling reps
    //we want to try multiple reps because some rep range might be closer to
    //expected_one_rep_max
    let min_acceptable_reps = ((exercise.upper_rep_range as f64) * 0.6).ceil() as usize;

    let mut min_rounding_error = f64::MAX;
    let mut weight_reps = (0.0, 0);

    for reps in min_acceptable_reps..=exercise.upper_rep_range {
        let coeff = rep_coefficients[reps - 1];
        let weight_at_reps = coeff * expected_one_rep_max;
        let rounding_error = weight_at_reps % exercise.minimum_weight_increment;
        if rounding_error < min_rounding_error {
            weight_reps = (weight_at_reps, reps);
            min_rounding_error = rounding_error;
        }
    }
    println!("{}x{}", weight_reps.0, weight_reps.1);
}
