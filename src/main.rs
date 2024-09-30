use anyhow::Result;
use clap::Parser;
use std::{
    collections::{hash_map::Iter, HashMap, HashSet},
    str::FromStr,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    source: String,
}

#[derive(Debug)]
struct TuringMachine {
    tape: Vec<String>,
    cursor: usize,
    init_state: String,
    accept_state: String,
    reject_state: String,
    transitions: HashMap<String, HashMap<String, (String, String, String)>>,
}

fn parse_tape(s: Vec<&str>) -> (Vec<&str>, usize) {
    // find the str starting with '*'
    let cursor = s.iter().position(|&x| x.starts_with('*')).unwrap();
    let tape = s
        .iter()
        .map(|&x| {
            if x.starts_with('*') {
                let mut chars = x.chars();
                chars.next(); // skip '*'
                chars.as_str()
            } else {
                x
            }
        })
        .collect();
    (tape, cursor)
}

fn parse_transitions(
    s: Vec<Vec<&str>>,
) -> HashMap<String, HashMap<String, (String, String, String)>> {
    let mut transitions = HashMap::new();
    for line in s {
        let state = line[0];
        let symbol = line[1];
        let new_symbol = line[2];
        let direction = line[3];
        let new_state = line[4];
        transitions
            .entry(state.to_string())
            .or_insert_with(HashMap::new)
            .insert(
                symbol.to_string(),
                (
                    new_symbol.to_string(),
                    direction.to_string(),
                    new_state.to_string(),
                ),
            );
    }
    transitions
}
impl FromStr for TuringMachine {
    type Err = anyhow::Error;
    fn from_str<'a>(s: &'a str) -> Result<Self> {
        let mut lines = s.lines();
        let scan_line =
            |line: &'a str| -> Vec<&'a str> { line.split_whitespace().collect::<Vec<_>>() };
        let tape = scan_line(lines.next().ok_or_else(|| anyhow::anyhow!("No tape"))?);
        let (tape, cursor) = parse_tape(tape);
        let tape = tape.into_iter().map(String::from).collect();
        let init_state = scan_line(
            lines
                .next()
                .ok_or_else(|| anyhow::anyhow!("No init state"))?,
        )[0]
        .to_string();
        let accept_state = scan_line(
            lines
                .next()
                .ok_or_else(|| anyhow::anyhow!("No accept states"))?,
        )[0]
        .to_string();
        let reject_state = scan_line(
            lines
                .next()
                .ok_or_else(|| anyhow::anyhow!("No reject states"))?,
        )[0]
        .to_string();
        let transitions = lines
            .into_iter()
            .map(scan_line)
            .collect::<Vec<Vec<&'a str>>>();
        let transitions = parse_transitions(transitions);
        Ok(Self {
            tape,
            cursor,
            init_state,
            accept_state,
            reject_state,
            transitions,
        })
    }
}

impl TuringMachine {
    fn list_states<'a>(&'a self) -> HashSet<&'a str> {
        let states = HashSet::from([
            self.init_state.as_str(),
            self.accept_state.as_str(),
            self.reject_state.as_str(),
        ]);
        states
            .union(&self.transitions.iter().map(|(k, _)| k.as_str()).collect())
            .copied()
            .collect()
    }
    fn list_symbols<'a>(&'a self) -> HashSet<&'a str> {
        let symbols: Vec<HashSet<&str>> = self
            .transitions
            .iter()
            .map(|(_, s)| s.iter().map(|(k, _)| k.as_str()).collect())
            .collect();
        symbols
            .iter()
            .fold(HashSet::new(), |acc, x| acc.union(x).copied().collect())
    }
    fn program_transition_function(
        &self,
        state_map: &HashMap<&str, usize>,
        symbol_map: &HashMap<&str, usize>,
    ) -> String {
        program_transition_function_state(&mut self.transitions.iter(), symbol_map, state_map)
    }
}

fn program_transition_function_state(
    state_iter: &mut Iter<'_, String, HashMap<String, (String, String, String)>>,
    symbol_map: &HashMap<&str, usize>,
    state_map: &HashMap<&str, usize>,
) -> String {
    if let Some((state, transitions)) = state_iter.next() {
        let mut code = String::new();
        code.push_str(&format!("if state == {} {{\n", state_map[state.as_str()]));
        code.push_str(&program_transition_fucntion_symbol(
            &mut transitions.iter(),
            symbol_map,
            state_map,
        ));
        if state_iter.len() == 0 {
            code.push_str("}\n");
        } else {
            code.push_str("} else {\n");
            code.push_str(&program_transition_function_state(
                state_iter, symbol_map, state_map,
            ));
            code.push_str("}\n");
        }
        code
    } else {
        String::new()
    }
}
fn program_transition_fucntion_symbol(
    symbol_iter: &mut Iter<'_, String, (String, String, String)>,
    symbol_map: &HashMap<&str, usize>,
    state_map: &HashMap<&str, usize>,
) -> String {
    if let Some((symbol, (new_symbol, direction, new_state))) = symbol_iter.next() {
        let mut code = String::new();
        code.push_str(&format!(
            "if symbol == {} {{\n",
            symbol_map[symbol.as_str()]
        ));
        if new_symbol.as_str() != "*" {
            code.push_str(&format!("symbol = {}\n", symbol_map[new_symbol.as_str()]));
        }
        match direction.as_str() {
            "L" => {
                code.push_str("move_left\n");
            }
            "R" => {
                code.push_str("move_right\n");
            }
            "*" => {}
            _ => panic!("Invalid direction"),
        }
        code.push_str(&format!("state = {}\n", state_map[new_state.as_str()]));
        if symbol_iter.len() == 0 {
            code.push_str("}\n");
        } else {
            code.push_str("} else {\n");
            code.push_str(&program_transition_fucntion_symbol(
                symbol_iter,
                symbol_map,
                state_map,
            ));
            code.push_str("}\n");
        }
        code
    } else {
        String::new()
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let source = std::fs::read_to_string(args.source)?;
    let tm = source.parse::<TuringMachine>()?;
    let states = tm.list_states();
    let symbols = tm.list_symbols();
    let state_map = states
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i))
        .collect::<HashMap<&str, usize>>();
    let symbol_map = symbols
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i))
        .collect::<HashMap<&str, usize>>();
    let code = vec![
        tm.tape
            .iter()
            .map(|sym| {
                let id = symbol_map[sym.as_str()];
                format!("symbol = {id}\nmove_right")
            })
            .collect::<Vec<String>>(),
        std::iter::repeat(String::from("move_left"))
            .take(tm.tape.len())
            .collect(),
        std::iter::repeat(String::from("move_right"))
            .take(tm.cursor)
            .collect(),
        vec![
            format!("state = {}", state_map[tm.init_state.as_str()]),
            format!(
                "while state != {} && state != {} {{",
                state_map[tm.accept_state.as_str()],
                state_map[tm.reject_state.as_str()]
            ),
            tm.program_transition_function(&state_map, &symbol_map),
            String::from("}"),
            format!(
                "if state == {} {{\nmsg = 89\noutput ( msg )\n}} else {{\nmsg = 78\noutput ( msg )\n}}\nmsg = 10\noutput ( msg )",
                state_map[tm.accept_state.as_str()]
            ),
        ],
    ]
    .iter()
    .flatten()
    .fold(String::new(), |acc, x| format!("{acc}\n{x}"));
    println!("{code}");
    Ok(())
}
