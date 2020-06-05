//  * ******************************************************************************************
//  * Copyright (c) 2019 Pascal Kuthe. This file is part of the OpenVAF project.
//  * It is subject to the license terms in the LICENSE file found in the top-level directory
//  *  of this distribution and at  https://gitlab.com/DSPOM/OpenVAF/blob/master/LICENSE.
//  *  No part of OpenVAF, including this file, may be copied, modified, propagated, or
//  *  distributed except according to the terms contained in the LICENSE file.
//  * *******************************************************************************************

use crate::analysis::IPDOM;
use crate::cfg::Terminator;
use crate::data_structures::BitSet;
use crate::ir::cfg::BasicBlockId;
use crate::ControlFlowGraph;
use index_vec::*;
use log::*;

pub type ControlDependenceGraph = IndexVec<BasicBlockId, BitSet<BasicBlockId>>;

impl ControlFlowGraph {
    /// Calculates the Control Dependence Graph of a CFG.
    /// A basic block is control dependent on a statement if this statements decides whether the block is executed (for example an if statement).
    /// Statements that cause control flow are not represented as statements but as basic block terminators in the cfg.
    /// As such the control dependence Graph simply maps basic blocks to all basics block whos terminator affats whether a statements is executed
    pub fn control_dependence_graph(&self) -> ControlDependenceGraph {
        self.control_dependence_graph_from_ipdom(&self.post_dominators())
    }

    /// The backend for [`control_dependence_graph`] which can be used when `ipdom(bb` has already been calcualted to avoid recalculation
    pub fn control_dependence_graph_from_ipdom(&self, ipdom: &IPDOM) -> ControlDependenceGraph {
        let mut cdg = index_vec![BitSet::new_empty(self.blocks.len_idx());self.blocks.len()];
        for (id, bb) in self.blocks.iter_enumerated() {
            // we only care about control dependencies on branches since end and goto are unconditional jumps
            if let Terminator::Split {
                true_block,
                false_block,
                merge,
                ..
            } = bb.terminator
            {
                self.propagate_control_dependence(&mut cdg, &ipdom, true_block, id);
                if merge != id {
                    self.propagate_control_dependence(&mut cdg, &ipdom, false_block, id);
                }
            }
        }

        cdg
    }

    fn propagate_control_dependence(
        &self,
        cdg: &mut ControlDependenceGraph,
        ipdom: &IPDOM,
        mut from: BasicBlockId,
        to: BasicBlockId,
    ) {
        loop {
            trace!(
                "propgating control dependency on {:?} to {:?} until {:?}",
                to,
                from,
                ipdom[to]
            );

            cdg[from].insert(to);
            from = ipdom[from];
            if from == ipdom[to] {
                break;
            }
        }
    }
}

#[cfg(feature = "graph_debug")]
mod print {

    use super::*;
    use rustc_ap_graphviz as dot;
    use rustc_ap_graphviz::LabelText::{EscStr, LabelStr};
    use rustc_ap_graphviz::{Edges, GraphWalk, Id, LabelText, Labeller, Nodes};
    use std::borrow::Cow;
    use std::io::Write;

    impl ControlFlowGraph {
        pub fn render_control_dependence_to<W: Write>(
            &self,
            write: &mut W,
            cdg: &ControlDependenceGraph,
        ) {
            dot::render(&ControlDependenceCFG(self, cdg), write).expect("Rendering failed")
        }
    }

    struct ControlDependenceCFG<'lt>(&'lt ControlFlowGraph, &'lt ControlDependenceGraph);
    #[derive(Copy, Clone, Eq, PartialEq)]
    enum FlowOrDependence {
        Flow,
        Dependence,
    }

    impl<'a> dot::Labeller<'a> for ControlDependenceCFG<'a> {
        type Node = BasicBlockId;
        type Edge = (FlowOrDependence, BasicBlockId, BasicBlockId);

        fn graph_id(&'a self) -> Id<'a> {
            dot::Id::new("ControlFlowGraph").unwrap()
        }

        fn node_id(&'a self, n: &Self::Node) -> Id<'a> {
            dot::Id::new(format!("BB_{}", n.index())).unwrap()
        }

        fn node_label(&'a self, &n: &Self::Node) -> LabelText<'a> {
            match self.0.blocks[n].terminator {
                Terminator::End => EscStr(Cow::Owned(format!(
                    "BB_{}: {} Statements\n END",
                    n.index(),
                    self.0.blocks[n].statements.len(),
                ))),
                Terminator::Goto(_) => LabelStr(Cow::Owned(format!(
                    "BB_{}: {} Statements",
                    n.index(),
                    self.0.blocks[n].statements.len(),
                ))),
                Terminator::Split {
                    condition, merge, ..
                } => LabelStr(Cow::Owned(format!(
                    "BB_{}: {} Statements\nSplit at {:?}\nMerge at BB_{}",
                    n.index(),
                    self.0.blocks[n].statements.len(),
                    condition,
                    merge.index(),
                ))),
            }
        }
        fn edge_label(
            &'a self,
            &(edge_type, start, dst): &(FlowOrDependence, BasicBlockId, BasicBlockId),
        ) -> LabelText<'a> {
            match edge_type {
                FlowOrDependence::Flow => match self.0[start].terminator {
                    Terminator::Goto(_) => LabelStr(Cow::Borrowed("GOTO")),
                    Terminator::End => LabelStr(Cow::Borrowed("ILLEGAL")),
                    Terminator::Split {
                        condition,
                        true_block,
                        false_block,
                        merge,
                    } => {
                        let true_or_false = if true_block == dst {
                            "TRUE"
                        } else if false_block == dst {
                            "FALSE"
                        } else {
                            "ILLEGAL"
                        };
                        LabelStr(Cow::Borrowed(true_or_false))
                    }
                },

                FlowOrDependence::Dependence => LabelStr(Cow::Borrowed("Control dependence")),
            }
        }
    }

    impl<'a> dot::GraphWalk<'a> for ControlDependenceCFG<'a> {
        type Node = BasicBlockId;
        type Edge = (FlowOrDependence, BasicBlockId, BasicBlockId);

        fn nodes(&'a self) -> Nodes<'a, Self::Node> {
            Cow::Owned(self.0.blocks.indices().collect())
        }

        fn edges(&'a self) -> Edges<'a, Self::Edge> {
            let mut edges = Vec::new();
            for (id, bb) in self.0.blocks.iter_enumerated() {
                match bb.terminator {
                    Terminator::Goto(dst) => edges.push((FlowOrDependence::Flow, id, dst)),
                    Terminator::Split {
                        condition,
                        true_block,
                        false_block,
                        merge,
                    } => {
                        edges.push((FlowOrDependence::Flow, id, false_block));
                        edges.push((FlowOrDependence::Flow, id, true_block));
                    }
                    Terminator::End => (),
                }
            }
            for (src, control_dependencies) in self.1.iter_enumerated() {
                for dst in control_dependencies.ones() {
                    edges.push((FlowOrDependence::Dependence, src, dst));
                }
            }
            Cow::Owned(edges)
        }

        fn source(&'a self, edge: &Self::Edge) -> Self::Node {
            edge.1
        }

        fn target(&'a self, edge: &Self::Edge) -> Self::Node {
            edge.2
        }
    }
}
