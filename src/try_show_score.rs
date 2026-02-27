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
            name,
            total_score,
            eng_score,
            math_score,
        }
    }

    pub fn score_alice() -> Self {
        Self { name: String::from("Alice"), total_score: 185, eng_score: 90, math_score: 95 }
    }

    pub fn score_bob() -> Self {
        Self { name: String::from("Bob"), total_score: 170, eng_score: 100, math_score: 70 }
    }
}