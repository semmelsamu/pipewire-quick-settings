use crate::models::sink::Sink;
use crate::models::state::PipeWireState;
use crate::printers;
use colored::*;
use serde_json::Value;
use std::io::{self, Write};

pub fn prompt(question: &str) -> String {
    print!("{} > ", question.blue().bold());
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

pub fn prompt_u32(question: &str) -> u32 {
    let input = prompt(question);
    input.trim().parse::<u32>().expect("Invalid input")
}

pub fn heading(text: &str) {
    println!();
    println!("{}", text.bold());
    println!("{}", "=".repeat(text.len()).bold());
}

pub fn value_as_u32(value: &Value) -> Option<u32> {
    match value {
        Value::Number(n) => n.as_u64().and_then(|n| u32::try_from(n).ok()),
        Value::String(s) => s.parse::<u32>().ok(),
        _ => None,
    }
}

pub fn prompt_sink(state: &PipeWireState, allow_default: bool) -> Sink {
    printers::sinks(state);

    let sink_id: u32;

    if allow_default {
        let input = prompt("Choose a sink (leave empty for default)");

        if input.trim().is_empty() {
            println!("Chosing default sink");
            sink_id = state.default_sink.clone().unwrap().id;
        } else {
            sink_id = input.trim().parse::<u32>().expect("Invalid sink id");
        }
    } else {
        sink_id = prompt_u32("Choose a sink");
    }

    let sink = state
        .sinks
        .iter()
        .find(|s| s.id == sink_id)
        .expect("Sink not found")
        .clone();

    println!("Chose sink {} ({})", sink.id, sink.description);
    sink
}
