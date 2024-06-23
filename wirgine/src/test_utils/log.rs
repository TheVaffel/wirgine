
use colored::{Colorize, ColoredString};

pub struct Log {}

fn prefix() -> String {
    String::from("[wirgine-test]")
}

fn info_prefix() -> ColoredString {
    String::from("INFO:").blue()
}

fn success_prefix() -> ColoredString {
    String::from("SUCCESS:").green()
}

fn fail_prefix() -> ColoredString {
    String::from("FAIL:").red()
}


impl Log {
    pub fn info(str: &String) {
        println!("{} {}: {}", &prefix(), &info_prefix(), str);
    }

    pub fn fail(str: &String) {
        println!("{} {}: {}", &prefix(), &fail_prefix(), str);
    }

    pub fn success(str: &String) {
        println!("{} {}: {}", &prefix(), &success_prefix(), str);
    }
}
