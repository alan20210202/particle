pub mod automatons;
pub mod lexer;

fn main() {
    let nfa = automatons::NFA::from(('0', '9'));
    println!("{:?}", nfa);
    println!("{:?}", nfa.not());
}
