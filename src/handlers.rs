use axum;
use axum::response::IntoResponse;
use axum::response::Json;
use tracing::{instrument, Span};

use crate::models::Calculation;

#[instrument("calculation_part1", skip_all, fields(outcome))]
fn calculation_part_1() {
    calculation_part_1_1();
    calculation_part_1_2();
}

#[instrument("calculation_part1_1", skip_all, fields(outcome))]
fn calculation_part_1_1() {}

#[instrument("calculation_part1_2", skip_all, fields(outcome))]
fn calculation_part_1_2() {}

#[instrument("calculation_part2", skip_all, fields(outcome))]
fn calculation_part_2() {}

//#[instrument("calculation", skip_all, fields(outcome))]
pub async fn calculate() -> impl IntoResponse {
    Span::current().record("outcome", "failure");
    calculation_part_1();
    calculation_part_2();
    let result = Calculation::new(42);
    Span::current().record("outcome", "success");
    return Json(result);
}

// #[instrument("panic", skip_all, fields(outcome))]
pub async fn create_panic() -> impl IntoResponse {
    Span::current().record("outcome", "failure");
    panic!("Created a panic!");
}
