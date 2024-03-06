// Author: Timotej Ponek, timotej.ponek@gmail.com
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{App, Arg};

fn main() {
    let matches: clap::ArgMatches = App::new("My Rust Project")
        .arg(
            Arg::with_name("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Sets the input file")
                .takes_value(true),
        )
        .get_matches();

    let file_path: Option<&str> = matches.value_of("file");

    let questions: Vec<String> = if let Some(file_path) = file_path {
        read_questions_from_file(file_path)
    } else {
        read_questions_from_stdin()
    };

    const MODULO_CONSTANT: u64 = u64::pow(10, 9) + 7;

    questions.iter().for_each(|question: &String| {
        
        let mut array: [u64; 5] = [0, 1, 2, 3, 5];
        let num_iterations: usize = question.trim().parse().expect("Invalid number");

        if num_iterations < 4 || num_iterations > 10_000_000_000 {
            panic!("Invalid question - not within bounds 4<=question<=10^10")
        }

        for _ in 4..num_iterations {
            let res: u64 = (array[1] + array[3] + array[4]) % MODULO_CONSTANT;
            array[0] += array[1];
            array[1] = array[2];
            array[2] = array[3];
            array[3] = array[4];
            array[4] = res;
        }

        let sum: u64 = array.iter().sum();
        // if number too big, just list its remainder after dividing by 10^9 + 7.
        println!("{}", sum % MODULO_CONSTANT);
    });
}

fn read_questions_from_file(file_path: &str) -> Vec<String> {
    let file: File = File::open(file_path).expect("Failed to open the file");
    let reader: BufReader<File> = BufReader::new(file);

    let mut questions: Vec<String> = Vec::new();

    reader.lines().skip(1).for_each(|line: Result<String, io::Error>| {
        if let Ok(line) = line {
            questions.push(line);
        }
    });

    questions
}

fn read_questions_from_stdin() -> Vec<String> {
    let mut input: String = String::new();

    println!("Enter the number of questions:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let num_questions: usize = input.trim().parse().expect("Invalid number");

    let mut questions: Vec<String> = Vec::new();

    for _ in 0..num_questions {
        let mut question: String = String::new();
        io::stdin()
            .read_line(&mut question)
            .expect("Failed to read input");
        questions.push(question.trim().to_string());
    }

    questions
}
