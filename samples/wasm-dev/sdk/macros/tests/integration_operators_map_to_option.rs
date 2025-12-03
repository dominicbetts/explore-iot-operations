// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

struct UserType {}
fn decode_input(_input: &DataModel) -> UserType {
    UserType {}
}
fn encode_output(input: DataModel, _user_output: UserType) -> DataModel {
    input
}
#[sdk_macros::map_operator(decode = "decode_input", encode = "encode_output")]
fn test_map(_input: UserType) -> Option<UserType> {
    None
}

#[test]
fn test_integration_wit_bindgen() {}
