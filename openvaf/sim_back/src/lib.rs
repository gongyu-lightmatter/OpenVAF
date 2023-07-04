pub use compilation_db::CompilationDB;
use hir_def::NodeId;
use hir_lower::{CurrentKind, ImplicitEquation};
pub use middle::{BoundStepKind, CacheSlot, EvalMir};
pub use module_info::ModuleInfo;
pub use residual::Residual;
use stdx::impl_debug_display;

mod compilation_db;
mod lim_rhs;
pub mod matrix;
mod middle;
mod module_info;
mod prune;
pub mod residual;

mod util;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum SimUnknown {
    KirchoffLaw(NodeId),
    Current(CurrentKind),
    Implicit(ImplicitEquation),
}

impl_debug_display! {
    match SimUnknown{
        SimUnknown::KirchoffLaw(node) => "{node}";
        SimUnknown::Current(curr) => "br[{curr:?}]";
        SimUnknown::Implicit(node) => "{node}";
    }
}
