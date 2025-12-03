// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

mod mod1 {
    sdk_wit::wasmtime_bindgen!("tinykube-graph:processor/host");
}

mod mod2 {
    #[sdk_wit::with_default_path]
    wasmtime::component::bindgen!("tinykube-graph:processor/host");
}

#[test]
fn test_integration_wasmtime_bindgen() {}
