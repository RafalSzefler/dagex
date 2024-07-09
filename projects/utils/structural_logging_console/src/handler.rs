use std::{collections::HashMap, io::{self, IsTerminal}};

use immutable_string::ImmutableString;
use structural_logging::{models::{keys, LogDataHolder, SLObject}, traits::StructuralLogHandler};
use termcolor::{ColorChoice, StandardStream};

use crate::console_write::{ConsoleWrite, Context};

#[derive(Default)]
pub struct ConsoleHandler {
    cached_parsed_templates: HashMap<ImmutableString, Vec<ImmutableString>>,
}

fn read_text(input: &str) -> (usize, ImmutableString) {
    let mut chars = input.chars();
    let mut content = String::new();
    let mut offset = 0;

    loop {
        let Some(chr) = chars.next() else { break };

        offset += chr.len_utf8();

        if chr != '{' {
            content.push(chr);
            continue;
        }

        let Some(peek_next) = chars.next() else {
            content.push(chr);
            break;
        };

        if peek_next == '{' {
            content.push(chr);
            offset += peek_next.len_utf8();
            continue;
        }

        break;
    }

    let imm = ImmutableString::new(&content).unwrap();
    (offset, imm)
}

fn read_key(input: &str) -> (usize, ImmutableString) {
    if input.is_empty() {
        return (0, ImmutableString::empty().clone());
    }

    let mut chars = input.chars().peekable();
    let mut content = String::new();
    let mut offset = 0;

    loop {
        let Some(chr) = chars.peek() else { break };

        if !chr.is_whitespace() {
            break;
        }

        offset += chr.len_utf8();
        let _ = chars.next();
    }

    loop {
        let Some(chr) = chars.peek() else { break };

        if chr.is_whitespace() || *chr == '}' {
            break;
        }

        offset += chr.len_utf8();

        content.push(*chr);
        let _ = chars.next();
    }

    loop {
        let Some(chr) = chars.next() else { break };

        offset += chr.len_utf8();

        if chr.is_whitespace() {
            continue;
        }

        if chr == '}' {
            break;
        }

        panic!("Invalid template key.");
    }

    let imm = ImmutableString::new(&content).unwrap();
    (offset, imm)
}

fn parse_template(template: &ImmutableString) -> Vec<ImmutableString> {
    if template.is_empty() {
        return Vec::default();
    }
    
    let mut txt = template.as_str();
    let mut result = Vec::with_capacity(4);
    while !txt.is_empty() {
        let (read, piece) = read_text(txt);
        result.push(piece);
        let current_len = txt.len();
        txt = &txt[read..current_len];

        let (read, piece) = read_key(txt);
        result.push(piece);
        let current_len = txt.len();
        txt = &txt[read..current_len];
    }

    result
}

impl StructuralLogHandler for ConsoleHandler {
    fn handle(&mut self, log: &LogDataHolder) {
        let data = log.log_data();
        if data.is_empty() {
            return;
        }

        let template = match &data[&keys::template()] {
            SLObject::String(slstring) => slstring.value(),
            _ => panic!("Invalid template type."),
        };

        let empty_map = HashMap::<ImmutableString, SLObject>::default();
        let template_params = if let Some(value) = data.get(&keys::template_params()) {
            match value {
                SLObject::Dict(data) => data.value(),
                _ => panic!("Invalid template_params type"),
            }
        } else {
            &empty_map
        };

        let parsed_template = {
            if let Some(value) = self.cached_parsed_templates.get(template) {
                value
            } else {
                let parsed = parse_template(template);
                self.cached_parsed_templates.insert(template.clone(), parsed);
                self.cached_parsed_templates.get(template).unwrap()
            }
        };

        if parsed_template.is_empty() {
            return;
        }

        let range = (0..parsed_template.len()).step_by(2);
        let is_terminal = io::stdout().is_terminal();
        let stdout = StandardStream::stdout(ColorChoice::Always);
        let guard = stdout.lock();
        
        let mut ctx = Context::new(guard, is_terminal);

        for idx in range {
            let text = &parsed_template[idx];
            text.write(&mut ctx);
            let key = &parsed_template[idx+1];
            if let Some(value) = template_params.get(key) {
                value.write(&mut ctx);
            } else if let Some(value) = data.get(key) {
                value.write(&mut ctx);
            }
        }

        ctx.flush();
    }
}
