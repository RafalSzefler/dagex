use std::marker::PhantomData;

use array::Array;
use dagex::core::{DirectedGraph, Node};

use crate::traits::{Algorithm, AlgorithmFactory};

pub struct DepthAlgorithm<'a> {
    graph: &'a DirectedGraph,
    scanned_nodes: Array<i32>,
}

pub struct DepthResult<'a> {
    max_depth: i32,
    phantom: PhantomData<&'a()>,
}

impl<'a> DepthResult<'a> {
    fn new(max_depth: i32) -> Self {
        Self { max_depth, phantom: PhantomData }
    }

    pub fn max_depth(&self) -> i32 { self.max_depth }
}

impl<'a> DepthAlgorithm<'a> {
    #[allow(clippy::cast_sign_loss)]
    fn scan(&mut self, node: Node) -> i32 {
        let idx = node.as_i32() as usize;

        {
            let scanned_nodes = self.scanned_nodes.as_slice_mut();
            let scanned_value = scanned_nodes[idx];
            if scanned_value != -1 {
                return scanned_value;
            }
        }

        let mut final_depth = -1;
        for child in self.graph.get_successors(node) {
            let child_depth = self.scan(*child);
            final_depth = core::cmp::max(final_depth, child_depth);
        }
        let final_value = final_depth + 1;
        self.scanned_nodes.as_slice_mut()[idx] = final_value;
        final_value
    }
}


impl<'a> Algorithm<'a> for DepthAlgorithm<'a> {
    type Input<'b> = &'b DirectedGraph;

    type Output<'b> = DepthResult<'b>;

    type Error = ();

    fn run(mut self) -> Result<Self::Output<'a>, Self::Error> {
        let root = self.graph.root().unwrap();
        let level = self.scan(root);
        Ok(DepthResult::new(level))
    }
}

#[derive(Debug)]
pub enum DepthInputValidationError {
    /// Input is not rooted.
    InputNotRooted,

    /// Input is not acyclic.
    InputNotAcyclic,

    /// Graph is too big. This algorithm allocates memory equal to
    /// `size_of::<i32>() * number_of_nodes` to achieve linear performance. We
    /// don't allow too big graphs thus. For max limit see
    /// [`DepthAlgorithmFactory::max_size`].
    GraphTooBig,
}

pub struct DepthAlgorithmFactory { }

impl DepthAlgorithmFactory {
    pub const fn max_size() -> usize { 1 << 30 }
}

impl AlgorithmFactory for DepthAlgorithmFactory {
    type Input<'a> = &'a DirectedGraph;

    type Algo<'a> = DepthAlgorithm<'a>;

    type Error = DepthInputValidationError;

    fn new() -> Self { Self {} }

    #[allow(clippy::cast_sign_loss)]
    fn create<'a>(&mut self, input: Self::Input<'a>)
        -> Result<Self::Algo<'a>, Self::Error>
    {
        let props = input.basic_properties();

        if !props.rooted {
            return Err(DepthInputValidationError::InputNotRooted);
        }

        if !props.acyclic {
            return Err(DepthInputValidationError::InputNotAcyclic);
        }
        
        let no = input.number_of_nodes() as usize;
        if no > Self::max_size() {
            return Err(DepthInputValidationError::GraphTooBig);
        }

        let scanned_nodes = Array::new_with_fill(no, || -1);

        Ok(DepthAlgorithm {
            graph: input,
            scanned_nodes: scanned_nodes,
        })
    }
}
