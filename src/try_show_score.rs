pub struct Test_score {
    pub name: String,
    pub total_score: i32,
    pub eng_score: i32,
    pub math_score: i32,
}

// inizialize.
impl Test_score{
    pub fn new(name: String, total_score: i32, eng_score: i32, math_score: i32) -> Self {
        Self {
            name: String::from(name),
            total_score,
            eng_score,
            math_score,
        }
    }
}