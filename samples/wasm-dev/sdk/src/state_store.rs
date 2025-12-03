// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

mod _gen {
    sdk_wit::wit_bindgen!("tinykube-graph:processor/state-store-use");
}
pub use _gen::tinykube_graph::processor::state_store::*;

// Tests
#[cfg(test)]
mod tests {

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn test_state_store_get() {
        use crate::state_store;
        let _ = state_store::get("key".as_bytes());
    }

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn test_state_store_set() {
        use crate::state_store;
        let _ = state_store::set("key".as_bytes(), "value".as_bytes());
    }
}
