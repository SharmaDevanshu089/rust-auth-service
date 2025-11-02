// yaha auth inset logic hoga
use serde::{Deserialize, Serialize};

// HANDLER SE YAHA LAA RAHA HU EASY NESS FOR PROGRAM
#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
