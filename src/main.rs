use std::time::SystemTime;

use clap::Parser;
use itertools::Itertools;

macro_rules! time {
    ($action:block) => {{
        let now = SystemTime::now();
        $action
        now.elapsed()
    }};
}

fn main() {
    let args = FlautomataArgs::parse();
    let mut line = args.start_line.map(|s| ensure_length(s, args.width, ' ')).unwrap_or(" ".repeat(args.width - 1) + "#");
    let mut res = line.clone() + "\n";
    let rule = Rule::new(args.rule);
    let time = time! {{
        for _ in 0..args.height {
            line = compute_next_state(line, &rule);
            res.push_str(&(line.clone() + "\n"))
        }
    }}.expect("Normal time on the machine");
    println!("{res}\n{time:?}");
}

fn compute_next_state(line: String, rule: &Rule) -> String {
    rotate(line)
        .into_chars()
        .map(|c| c == '#')
        .circular_tuple_windows()
        .map(|state| rule.check(state))
        .map(|b| if b { '#' } else { ' ' })
        .collect::<String>()
}

#[derive(Parser)]
struct FlautomataArgs {
    #[arg(short, long, default_value_t = 110)]
    rule: u8,
    #[arg(short = 'x', long, default_value_t = 30)]
    width: usize,
    #[arg(short = 'y', long, default_value_t = 30)]
    height: usize,
    #[arg(long, default_value = None)]
    start_line: Option<String>,
}

struct Rule {
    rule: [bool; 8],
}

impl Rule {
    fn new(rule: u8) -> Self {
        Self {
            rule: {
                let mut res = [false; 8];
                res.iter_mut().enumerate().for_each(|(i, v)| *v = (rule >> i) & 1 == 1);
                res
            }
        }
    }

    fn check(&self, state: (bool, bool, bool)) -> bool {
        let key = |(a, b, c)| ((a as usize) << 2) + ((b as usize) << 1) + c as usize;
        self.rule[key(state)]
    }
}

fn rotate(mut s: String) -> String {
    let Some(last) = s.pop() else { return String::new(); };
    s.insert(0, last);
    s
}

fn ensure_length(mut s: String, len: usize, fill: char) -> String {
    let l = s.len();
    match l {
        _ if l > len => {
            s = s.chars().take(len).collect()
        }
        _ if l < len => s.push_str(&fill.to_string().repeat(len - l)),
        _ => {}
    }
    s
}

trait IntoChars {
    fn into_chars(self) -> SizedChars;
}

impl IntoChars for String {
    fn into_chars(self) -> SizedChars {
        SizedChars {
            str: self
        }
    }
}

#[derive(Clone)]
struct SizedChars {
    str: String,
}

impl Iterator for SizedChars {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.str.is_empty() { None } else { Some(self.str.remove(0)) }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.str.len(), Some(self.str.len()))
    }
}

impl ExactSizeIterator for SizedChars {}