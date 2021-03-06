// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use dep_graph::DepNode;
use util::nodemap::NodeMap;
use mir::repr::Mir;
use mir::transform::MirPass;
use middle::ty::{self, TyCtxt};
use middle::infer;

pub struct MirMap<'tcx> {
    pub map: NodeMap<Mir<'tcx>>,
}

impl<'tcx> MirMap<'tcx> {
    pub fn run_passes(&mut self, passes: &mut [Box<MirPass>], tcx: &TyCtxt<'tcx>) {
        if passes.is_empty() { return; }

        for (&id, mir) in &mut self.map {
            let did = tcx.map.local_def_id(id);
            let _task = tcx.dep_graph.in_task(DepNode::MirMapConstruction(did));

            let param_env = ty::ParameterEnvironment::for_item(tcx, id);
            let infcx = infer::new_infer_ctxt(tcx, &tcx.tables, Some(param_env));

            for pass in &mut *passes {
                pass.run_on_mir(mir, &infcx)
            }
        }
    }
}
