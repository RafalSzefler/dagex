use immutable_string::ImmutableString;

use super::TemplatePiece;

fn read_text(input: &str) -> (usize, ImmutableString) {
    let mut chars = input.chars();
    let mut content = String::new();
    let mut offset = 0;

    loop {
        let Some(chr) = chars.next() else { break };

        if chr != '{' {
            offset += chr.len_utf8();
            content.push(chr);
            continue;
        }

        let Some(peek_next) = chars.next() else {
            offset += chr.len_utf8();
            content.push(chr);
            break;
        };

        if peek_next == '{' {
            offset += chr.len_utf8();
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
    let mut chars = input.chars().peekable();
    let _ = chars.next();
    let mut content = String::new();
    let mut offset = 1;

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

fn read_piece(txt: &str) -> (usize, TemplatePiece) {
    if txt.is_empty() {
        return (0, TemplatePiece::Empty);
    }

    let mut chars = txt.chars();

    if let Some(current) = chars.next() {
        if current == '{' {
            if let Some(next) = chars.next() {
                if next != '{' {
                    let (size, imm) = read_key(txt);
                    return (size, TemplatePiece::Parameter(imm));
                }
            }
        }
    }

    let (size, imm) = read_text(txt);
    (size, TemplatePiece::RawString(imm))
}

pub(super) fn parse_template_to_pieces(template: &ImmutableString) -> Vec<TemplatePiece> {
    if template.is_empty() {
        return Vec::default();
    }
    
    let mut txt = template.as_str();
    let mut result = Vec::with_capacity(4);
    while !txt.is_empty() {
        let (read, piece) = read_piece(txt);
        result.push(piece);
        let current_len = txt.len();
        txt = &txt[read..current_len];
    }

    result
}
