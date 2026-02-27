mod try_show_score;
use try_show_score::Test_score;
fn main() {
    let alice_score = Test_score::new(String::from("Alice"), 185, 90, 95);
    let bob_score = Test_score::new(String::from("Bob"), 170, 100, 70);

    println!("{}´s total score: {}, English: {}, math:{}   ", alice_score.name, alice_score.total_score, alice_score.eng_score, alice_score.math_score);
    println!("{}´s total score: {}, English: {}, math:{}   ", bob_score.name, bob_score.total_score, bob_score.eng_score, bob_score.math_score);
}
