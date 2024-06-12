use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Calculation {
    result: u64,
}

impl Calculation {
    pub fn new(result: u64) -> Calculation {
        Calculation { result: result }
    }
}
