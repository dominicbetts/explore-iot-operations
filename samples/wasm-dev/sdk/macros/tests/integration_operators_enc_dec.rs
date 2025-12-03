// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

mod test_map {
    struct UserType {}
    fn decode_input(_input: &DataModel) -> UserType {
        UserType {}
    }
    fn encode_output(input: DataModel, _user_output: UserType) -> DataModel {
        input
    }
    #[sdk_macros::map_operator(decode = "decode_input", encode = "encode_output")]
    fn test_map(input: UserType) -> UserType {
        input
    }
}

mod test_filter {
    struct UserType {}
    fn decode_input(_input: &DataModel) -> UserType {
        UserType {}
    }
    #[sdk_macros::filter_operator(decode = "decode_input")]
    fn test_filter(_input: UserType) -> bool {
        true
    }
}

mod test_branch {
    struct UserType {}
    fn decode_input(_input: &DataModel) -> UserType {
        UserType {}
    }
    #[sdk_macros::branch_operator(decode = "decode_input")]
    fn test_branch(_timestamp: HybridLogicalClock, _message: UserType) -> bool {
        false
    }
}

#[test]
fn test_integration_wit_bindgen() {}
