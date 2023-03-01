use std::env;
use std::process;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Note {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
}

impl FromStr for Note {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Note::C),
            "Db" => Ok(Note::Db),
            "D" => Ok(Note::D),
            "Eb" => Ok(Note::Eb),
            "E" => Ok(Note::E),
            "F" => Ok(Note::F),
            "Gb" => Ok(Note::Gb),
            "G" => Ok(Note::G),
            "Ab" => Ok(Note::Ab),
            "A" => Ok(Note::A),
            "Bb" => Ok(Note::Bb),
            "B" => Ok(Note::B),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Config {
    root: Note,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("Only one argument (root note) is supported");
        }
        let root = Note::from_str(&args[1]).unwrap();
        Ok(Config { root })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Wrong input: {err}");
        process::exit(1);
    });
    println!("the root note is {:?}", config.root);
}
