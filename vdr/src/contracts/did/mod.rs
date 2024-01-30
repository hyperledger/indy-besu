pub mod did_ethr_registry;
pub mod types;

pub use did_ethr_registry::*;
pub(crate) use types::did_doc_builder::DidDocumentBuilder;
pub use types::{did::DID, did_doc::*};
