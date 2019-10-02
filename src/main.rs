const USAGE: &'static str = "
Usage: mina [options]

Options:
    --input <string>    File for reading a sequence of arrival times
    --output <string>   File for writing a sequence of arrival times
    --length <number>   Number of arrival times to be generated
    --seed <number>     Seed for the random number generator
    --help              Flag for showing this help message
";

use fractal::Beta;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, BufRead, Write};

pub struct Fractal {
    time: f64,
    model: Beta,
    arrivals: VecDeque<f64>,
    source: Source,
}

pub type Source = random::Default;

pub trait Traffic {
    fn next(&mut self) -> Option<f64>;
    fn peek(&mut self) -> Option<&f64>;
}

impl Fractal {
    pub fn new(data: &[f64], source: Source) -> Fractal {
        let blocks = match (data.len() as f64).log2().floor() {
            blocks if blocks < 1.0 => panic!("there are not enough data"),
            blocks => blocks as usize,
        };
        Fractal {
            time: 0.0,
            model: Beta::new(data, blocks).unwrap(),
            arrivals: VecDeque::new(),
            source: source,
        }
    }

    fn refill(&mut self) {
        for step in self.model.sample(&mut self.source).unwrap() {
            self.time += step;
            self.arrivals.push_back(self.time);
        }
    }
}

impl Traffic for Fractal {
    fn next(&mut self) -> Option<f64> {
        if self.arrivals.is_empty() {
            self.refill();
        }
        self.arrivals.pop_front()
    }

    fn peek(&mut self) -> Option<&f64> {
        if self.arrivals.is_empty() {
            self.refill();
        }
        self.arrivals.get(0)
    }
}

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    if arguments.get::<bool>("help").unwrap_or(false) {
        println!("{}", USAGE.trim());
        return;
    }
    let seed = arguments
        .get::<i64>("seed")
        .map(|seed| seed as u64)
        .unwrap_or(42);
    let seed = [0x12345678 ^ seed, 0x87654321 ^ seed];
    let source = random::default().seed(seed);
    let input = arguments.get::<String>("input").unwrap();
    let input = File::open(input).unwrap();
    let input = BufReader::new(input);
    let mut data = vec![];
    let mut last = None;
    for line in input.lines() {
        let value = line.unwrap().parse::<f64>().unwrap();
        if last.is_some() {
            data.push(value - last.unwrap());
        }
        last = Some(value);
    }
    let mut model = Fractal::new(&data, source);
    let output = arguments.get::<String>("output").unwrap();
    let mut output = File::create(output).unwrap();
    for _ in 0..arguments.get::<i64>("length").map(|length| length as u64).unwrap() {
        writeln!(output, "{}", model.next().unwrap()).unwrap();
    }
}
