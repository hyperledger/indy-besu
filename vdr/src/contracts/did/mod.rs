pub mod did_ethr_registry;
pub mod types;

pub(crate) mod did_ethr_resolver;

pub use did_ethr_registry::*;
pub use types::{did::DID, did_doc::*, did_doc_attribute::*};
