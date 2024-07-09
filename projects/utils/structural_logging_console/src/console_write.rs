use std::{collections::HashMap, io::Write, time::SystemTime};

use chrono::{DateTime, SecondsFormat, Utc};
use immutable_string::ImmutableString;
use structural_logging::{models::SLObject, traits::LogLevel};
use termcolor::{Color, ColorSpec, StandardStreamLock, WriteColor};

pub struct Context<'a> {
    stdout: StandardStreamLock<'a>,
    is_terminal: bool,
}

impl<'a> Context<'a> {
    pub fn new(stdout: StandardStreamLock<'a>, is_terminal: bool) -> Self {
        Self { stdout, is_terminal }
    }
    
    pub fn write(&mut self, txt: &str, color: &ColorSpec) {
        if self.is_terminal {
            self.stdout.set_color(color).unwrap();
        }
        self.stdout.write_all(txt.as_bytes()).unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.write_all(b"\n").unwrap();
        if self.is_terminal {
            self.stdout.reset().unwrap();
        }
        self.stdout.flush().unwrap();
    }
}

pub trait ConsoleWrite {
    fn write(&self, ctx: &mut Context);
}

#[inline(always)]
fn color_spec(color: Color) -> ColorSpec {
    let mut spec: ColorSpec = ColorSpec::new();
    spec.set_fg(Some(color));
    spec.set_intense(true);
    spec
}

impl ConsoleWrite for ImmutableString {
    fn write(&self, ctx: &mut Context) {
        ctx.write(self.as_str(), &color_spec(Color::Ansi256(7)));
    }
}

impl ConsoleWrite for LogLevel {
    fn write(&self, ctx: &mut Context) {
        match self {
            LogLevel::Debug => {
                let color = {
                    let mut spec: ColorSpec = ColorSpec::new();
                    spec.set_fg(Some(Color::White));
                    spec.set_dimmed(true);
                    spec.set_intense(false);
                    spec
                };
                ctx.write("DEBUG", &color);
            }
            LogLevel::Info => {
                ctx.write("INFO", &color_spec(Color::Yellow));
            },
            LogLevel::Warning => {
                ctx.write("WARNING", &color_spec(Color::Magenta));
            }
            LogLevel::Error => {
                ctx.write("ERROR", &color_spec(Color::Red));
            }
        }
    }
}

impl ConsoleWrite for SystemTime {
    fn write(&self, ctx: &mut Context) {
        let dt: DateTime<Utc> = (*self).into();
        let text = dt.to_rfc3339_opts(SecondsFormat::Secs, true);
        let color = {
            let mut spec: ColorSpec = ColorSpec::new();
            spec.set_fg(Some(Color::White));
            spec.set_intense(true);
            spec
        };
        ctx.write(&text, &color);
    }
}

impl ConsoleWrite for std::time::Duration {
    #[allow(clippy::cast_possible_truncation)]
    fn write(&self, ctx: &mut Context) {
        let total_millis = self.as_millis();
        let mut millis = (total_millis % 1000) as u16;
        let mut secs = (total_millis / 1000) as u64;
        let mut buffer = [0u8; 46];
        let mut offset = 0usize;
        loop {
            buffer[offset] = b'0' + ((secs % 10) as u8);
            offset += 1;
            secs /= 10;
            if secs == 0 {
                break;
            }
        }
        buffer[0..offset].reverse();

        buffer[offset] = b'.';
        offset += 1;
        let start = offset;
        loop {
            buffer[offset] = b'0' +((millis % 10) as u8);
            offset += 1;
            millis /= 10;
            if millis == 0 {
                break;
            }
        }
        while offset < start + 3 {
            buffer[offset] = b'0';
            offset += 1;
        }

        buffer[start..offset].reverse();

        buffer[offset] = b's';
        offset += 1;

        let buffer_slice = &buffer[0..offset];
        let slice = unsafe { core::str::from_utf8_unchecked(buffer_slice) };
        ctx.write(slice, &color_spec(Color::Cyan));
    }
}

impl ConsoleWrite for i64 {
    fn write(&self, ctx: &mut Context) {
        ctx.write(&self.to_string(), &color_spec(Color::Blue));
    }
}

impl ConsoleWrite for bool {
    fn write(&self, ctx: &mut Context) {
        match self {
            true => ctx.write("true", &color_spec(Color::Magenta)),
            false => ctx.write("false", &color_spec(Color::Magenta)),
        }
    }
}

impl ConsoleWrite for Vec<SLObject> {
    fn write(&self, ctx: &mut Context) {
        ctx.write("[", &color_spec(Color::Yellow));
        let mut iter = self.iter();
        if let Some(obj) = iter.next() {
            obj.write(ctx);
            for item in iter {
                ctx.write(", ", &color_spec(Color::Yellow));
                item.write(ctx);
            }
        }
        ctx.write("]", &color_spec(Color::Yellow));
    }
}

impl ConsoleWrite for HashMap<ImmutableString, SLObject> {
    fn write(&self, ctx: &mut Context) {
        ctx.write("{", &color_spec(Color::Yellow));
        let mut iter = self.iter();
        if let Some(obj) = iter.next() {
            obj.0.write(ctx);
            ctx.write(": ", &color_spec(Color::Yellow));
            obj.1.write(ctx);

            for item in iter {
                ctx.write(", ", &color_spec(Color::Yellow));
                item.0.write(ctx);
                ctx.write(": ", &color_spec(Color::Yellow));
                item.1.write(ctx);
            }
        }
        ctx.write("}", &color_spec(Color::Yellow));
    }
}

impl ConsoleWrite for SLObject {
    fn write(&self, ctx: &mut Context) {
        match self {
            SLObject::LogLevel(inner) => inner.value().write(ctx),
            SLObject::SystemTime(inner) => inner.value().write(ctx),
            SLObject::Duration(inner) => inner.value().write(ctx),
            SLObject::String(inner) => inner.value().write(ctx),
            SLObject::Number(inner) => inner.value().write(ctx),
            SLObject::Bool(inner) => inner.value().write(ctx),
            SLObject::Array(inner) => inner.value().write(ctx),
            SLObject::Dict(inner) => inner.value().write(ctx),
        }
    }
}
