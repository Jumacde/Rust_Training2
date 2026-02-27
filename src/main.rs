mod try_show_score;
mod try_calc;
use try_show_score::Test_score;
use try_calc:: Test_calc1;

fn main() {
    // show score.
    println!("<< show score>>");
    let alice_score = Test_score::score_alice();
    let bob_score = Test_score::score_bob();

    println!("{}´s total score: {}, English: {}, math:{}   ", alice_score.name, alice_score.total_score, alice_score.eng_score, alice_score.math_score);
    println!("{}´s total score: {}, English: {}, math:{}   ", bob_score.name, bob_score.total_score, bob_score.eng_score, bob_score.math_score);

    println!("\n << calc >>");
    let mut calc_case = Test_calc1::new(5, 7, 17);
    calc_case.calc_sum();
    calc_case.calc_mul();
    
}
