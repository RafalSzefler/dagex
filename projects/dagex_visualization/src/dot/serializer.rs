#![allow(clippy::cast_sign_loss)]
use std::{collections::{HashMap, HashSet}, hash::{Hash, Hasher}, io::{Error, Write}};

use dagex::core::DirectedGraphDTO;

use super::traits::{DotSerializable, DotSerializer};

pub struct DefaultDotSerializer<TWrite: Write> {
    stream: TWrite,
}

impl<TWrite: Write> From<TWrite> for DefaultDotSerializer<TWrite> {
    fn from(value: TWrite) -> Self {
        Self { stream: value }
    }
}

impl<TWrite: Write> DotSerializer<TWrite> for DefaultDotSerializer<TWrite> {
    fn release(self) -> TWrite { self.stream }
}

impl<TWrite> DotSerializable<TWrite, DefaultDotSerializer<TWrite>> for DirectedGraphDTO
    where TWrite: Write
{
    fn serialize(&self, ser: &mut DefaultDotSerializer<TWrite>) -> Result<usize, Error> {
        let s = &mut ser.stream;
        let graph = self;

        let hash = {
            let mut hasher = fnv1a_hasher::FNV1a32Hasher::new();
            graph.hash(&mut hasher);
            hasher.finish()
        };

        let mut total = 2;
        let mut write_it = |text: &String| -> Result<(), _> {
            let msg_buf = text.as_bytes();
            total += msg_buf.len();
            s.write_all(msg_buf)
        };

        write_it(&format!("digraph dagex_{hash} {{\n"))?;
        

        let no = graph.number_of_nodes();
        let mut arrows_map = HashMap::<i32, Vec<i32>>
            ::with_capacity(no as usize);
        let mut seen = HashSet::with_capacity(no as usize);

        for arrow in graph.arrows() {
            let src = arrow.source();
            let targets = if let Some(targs) = arrows_map.get_mut(&src) { targs } else {
                let targs = Vec::with_capacity(2);
                arrows_map.insert(src, targs);
                arrows_map.get_mut(&src).unwrap()
            };
            targets.push(arrow.target());
            seen.insert(src);
            seen.insert(arrow.target());
        }

        for idx in 0..no {
            match arrows_map.get(&idx) {
                Some(targets) => {
                    let targets_str = targets
                        .iter()
                        .map(std::string::ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" ");
                    write_it(&format!("    {idx} -> {{ {targets_str} }};\n"))?;
                },
                None => {
                    if !seen.contains(&idx) {
                        write_it(&format!("    {idx};\n"))?;
                    }
                },
            }
        }

        s.write_all(b"}\n")?;

        Ok(total)
    }
}
