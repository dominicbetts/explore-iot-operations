// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

mod _gen {
    sdk_wit::wit_bindgen!("tinykube-graph:processor/logger-use");
}
pub use _gen::tinykube_graph::processor::logger::*;

// Tests
#[cfg(test)]
mod tests {

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn test_logger() {
        use crate::logger;
        logger::log(logger::Level::Info, "context", "message");
    }
}
