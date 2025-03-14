use std::hash::BuildHasherDefault;

use rustc_hash::FxHasher;

use crate::lint::{LintRegistry, LintRegistryBuilder};
pub use db::Db;
pub use module_name::ModuleName;
pub use module_resolver::{resolve_module, system_module_search_paths, KnownModule, Module};
pub use program::{Program, ProgramSettings, SearchPathSettings, SitePackages};
pub use python_platform::PythonPlatform;
pub use python_version::PythonVersion;
pub use semantic_model::{HasTy, SemanticModel};

pub mod ast_node_ref;
mod db;
pub mod lint;
mod module_name;
mod module_resolver;
mod node_key;
mod program;
mod python_platform;
mod python_version;
pub mod semantic_index;
mod semantic_model;
pub(crate) mod site_packages;
mod stdlib;
mod suppression;
pub(crate) mod symbol;
pub mod types;
mod unpack;
mod util;
mod visibility_constraints;

type FxOrderSet<V> = ordermap::set::OrderSet<V, BuildHasherDefault<FxHasher>>;

/// Creates a new registry with all known semantic lints.
pub fn default_lint_registry() -> LintRegistry {
    let mut registry = LintRegistryBuilder::default();
    register_lints(&mut registry);
    registry.build()
}

/// Register all known semantic lints.
pub fn register_lints(registry: &mut LintRegistryBuilder) {
    types::register_lints(registry);
}
