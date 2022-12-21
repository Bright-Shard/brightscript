use brightscript::*;

fn main() {
    let mut util = OSUtil::new();

    util.exec(String::from("echo hi")).unwrap();
}
