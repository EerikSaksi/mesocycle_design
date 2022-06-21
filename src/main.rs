use csv::Writer;
use serde::Serialize;
mod set_generator;

#[derive(Debug, Serialize)]
pub struct Exercise<'a> {
    name: &'a str,
    current_training_max: f64,
    max_increase_coefficient: f64,
    minimum_weight_increment: f64,
    upper_rep_range: u32,
}

//impl Default for Exercise {
//    fn default() -> Self {
//        Exercise {
//            name: "",
//            current_training_max: 0.0,
//            training_min_coefficient: 0.95,
//            training_max_coefficient: 1.05,
//            upper_rep_range: 0,
//            minimum_weight_increment: 0.0,
//        }
//    }
//}

struct Plan<'a> {
    push_exercises: Vec<Exercise<'a>>,
    pull_exercises: Vec<Exercise<'a>>,
    num_weeks: u32,
    microcycle_days: u32,
}
fn one_rep_max(weight: f64, reps: u32) -> f64 {
    weight * (1.0 + ((reps as f64) / 30.0))
}

fn main() {
    let plan = Plan {
        push_exercises: vec![
            Exercise {
                minimum_weight_increment: 2.0,
                upper_rep_range: 12,
                max_increase_coefficient: 0.05,
                current_training_max: one_rep_max(32.0, 8),
                name: "Incline Dumbbell Bench",
            },
            Exercise {
                minimum_weight_increment: 1.0,
                upper_rep_range: 8,
                max_increase_coefficient: 0.05,
                current_training_max: one_rep_max(100.0, 5),
                name: "Front Squat",
            },
            Exercise {
                minimum_weight_increment: 1.0,
                upper_rep_range: 8,
                max_increase_coefficient: 0.05,
                current_training_max: one_rep_max(100.0, 5),
                name: "Front Squat",
            },
        ],
        pull_exercises: vec![],
        microcycle_days: 4,
        num_weeks: 6,
    };
}
