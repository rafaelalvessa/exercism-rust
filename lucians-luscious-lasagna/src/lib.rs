use std::cmp;

/// Time it takes to prepare a lasagna sheet in minutes.
const LAYER_PREPARATION_TIME_IN_MINUTES: i32 = 2;

/// Time it takes to cook the lasagna in the oven in minutes.
const OVEN_TIME_IN_MINUTES: i32 = 40;

/// Returns the expected time in the oven in minutes.
pub fn expected_minutes_in_oven() -> i32 {
    OVEN_TIME_IN_MINUTES
}

/// Returns the minutes left in the oven given the time it has been cooking for.
pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    cmp::max(expected_minutes_in_oven() - actual_minutes_in_oven, 0)
}

/// Returns the preparation time in minutes for the given number of lasagna layers.
pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    number_of_layers * LAYER_PREPARATION_TIME_IN_MINUTES
}

/// Returns the time in minutes that it has taken to prepare and cook the lasagna so far.
pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
