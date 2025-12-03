// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

mod mod1 {
    sdk_wit::wit_bindgen!("tinykube-graph:processor/logger-use");
}

mod mod2 {
    #[sdk_wit::with_default_path]
    wit_bindgen::generate!("tinykube-graph:processor/logger-use");
}

mod mod3 {
    #[sdk_wit::with_default_path]
    wit_bindgen::generate!({world: "tinykube-graph:processor/logger-use"});
}

#[test]
fn test_integration_wit_bindgen() {}
