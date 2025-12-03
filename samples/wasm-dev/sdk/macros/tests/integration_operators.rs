// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

mod test_map {
    #[sdk_macros::map_operator]
    fn test_map(input: DataModel) -> DataModel {
        input
    }
}

mod test_filter {
    #[sdk_macros::filter_operator]
    fn test_filter(_input: DataModel) -> bool {
        true
    }
}

mod test_branch {
    #[sdk_macros::branch_operator]
    fn test_branch(_timestamp: HybridLogicalClock, _message: DataModel) -> bool {
        false
    }
}

mod test_delay {
    #[sdk_macros::delay_operator]
    fn test_delay(_data: DataModel, timestamp: HybridLogicalClock) -> HybridLogicalClock {
        timestamp
    }
}

mod test_accumulate {
    #[sdk_macros::accumulate_operator]
    fn test_accumulate(staged: DataModel, _input: Vec<DataModel>) -> DataModel {
        staged
    }
}

#[test]
fn test_integration_wit_bindgen() {}
