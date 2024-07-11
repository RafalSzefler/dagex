use std::{collections::HashMap, sync::{Arc, Mutex, OnceLock}};

use raf_immutable_string::ImmutableString;

use super::parser::parse_template_to_pieces;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum TemplatePiece {
    Empty,
    RawString(ImmutableString),
    Parameter(ImmutableString),
}

impl Default for TemplatePiece {
    fn default() -> Self { Self::Empty }
}

#[derive(Default, Clone, Debug)]
pub struct Template {
    raw: ImmutableString,
    pieces: Arc<Vec<TemplatePiece>>,
}


impl PartialEq for Template {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl Eq for Template { }

impl core::hash::Hash for Template {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl Template {
    fn new(raw: ImmutableString, pieces: Arc<Vec<TemplatePiece>>) -> Self {
        Self { raw, pieces }
    }

    pub fn as_immutable_string(&self) -> &ImmutableString { &self.raw }

    pub fn pieces(&self) -> &[TemplatePiece] { &self.pieces }
}

#[derive(Default)]
pub struct TemplateBuilder {
    cache: Mutex<HashMap<ImmutableString, Template>>,
}

impl TemplateBuilder {
    pub fn create(&self, txt: &str) -> Template {
        let imm = ImmutableString::new(txt).unwrap();

        {
            let guard = self.cache.lock().unwrap();
            if let Some(tmpl) = guard.get(&imm) {
                return tmpl.clone();
            }
        }

        let pieces = parse_template_to_pieces(&imm);
        let new_tmpl = Template::new(
            imm.clone(), 
            Arc::new(pieces));

        {
            let mut guard = self.cache.lock().unwrap();
            if let Some(tmpl) = guard.get(&imm) {
                return tmpl.clone();
            }
            guard.insert(imm, new_tmpl.clone());
        }

        new_tmpl
    }
}


static LOGGER_NAME: OnceLock<ImmutableString> = OnceLock::new();
pub fn get_logger_name_key() -> &'static ImmutableString {
    LOGGER_NAME.get_or_init(|| {
        ImmutableString::new("logger_name").unwrap()
    })
}
