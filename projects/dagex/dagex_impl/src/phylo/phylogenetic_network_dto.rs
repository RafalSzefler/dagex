use std::collections::HashMap;

use raf_readonly::readonly;

use crate::raf_array::immutable_string::ImmutableString;

use crate::core::DirectedGraphDTO;

#[readonly]
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct PhylogeneticNetworkDTO {
    pub graph: DirectedGraphDTO,
    pub taxa: HashMap<i32, ImmutableString>,
}
