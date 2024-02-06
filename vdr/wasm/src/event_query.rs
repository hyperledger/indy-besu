use indy_besu_vdr::EventQuery;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = EventQuery)]
pub struct EventQueryWrapper(pub(crate) Rc<EventQuery>);
