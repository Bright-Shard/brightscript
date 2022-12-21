use brightscript::*;

fn main() {
    let mut bs = Parser::new();

    bs.parse(String::from(concat!(
        "$ls;",
        "$echo hi;",
        "$echo bye;",
        "$echo 'hi' >> thing.txt;"
    )));
}
