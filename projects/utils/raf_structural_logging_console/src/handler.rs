use std::io::{self, IsTerminal};

use raf_structural_logging::{
    models::LogDataHolder,
    template::TemplatePiece,
    traits::StructuralLogHandler};
use termcolor::{ColorChoice, StandardStream};

use crate::console_write::{ConsoleWrite, Context};

#[derive(Default)]
pub struct ConsoleHandler;


impl StructuralLogHandler for ConsoleHandler {
    fn handle(&self, log: &LogDataHolder) {
        if log.is_empty() {
            return;
        }

        let template = log.template();
        let template_params = log.template_params();
        let additional_data = log.additional_data();

        let is_terminal = io::stdout().is_terminal();
        let stdout = StandardStream::stdout(ColorChoice::Always);
        let guard = stdout.lock();
        
        let mut ctx = Context::new(guard, is_terminal);

        for piece in template.pieces() {
            match piece {
                TemplatePiece::RawString(txt) => {
                    txt.write(&mut ctx);
                },
                TemplatePiece::Parameter(txt) => {
                    if let Some(value) = template_params.get(txt) {
                        value.write(&mut ctx);
                    } else if let Some(value) = additional_data.get(txt) {
                        value.write(&mut ctx);
                    }
                },
                TemplatePiece::Empty => { },
            }
        }

        ctx.flush();
    }
}
