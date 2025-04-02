// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock
//
// This file was autogenerated from Opc.Ua.Types.bsd by tools/schema/gen_types.js
//
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]
#[allow(unused_imports)]
use crate::types::{
    basic_types::*, encoding::*, node_ids::ObjectId, request_header::RequestHeader,
    service_types::impls::MessageInfo,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct TransferSubscriptionsRequest {
    pub request_header: RequestHeader,
    pub subscription_ids: Option<Vec<u32>>,
    pub send_initial_values: bool,
}

impl MessageInfo for TransferSubscriptionsRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::TransferSubscriptionsRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<TransferSubscriptionsRequest> for TransferSubscriptionsRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += byte_len_array(&self.subscription_ids);
        size += self.send_initial_values.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += write_array(stream, &self.subscription_ids)?;
        size += self.send_initial_values.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_options)?;
        let subscription_ids: Option<Vec<u32>> = read_array(stream, decoding_options)?;
        let send_initial_values = bool::decode(stream, decoding_options)?;
        Ok(TransferSubscriptionsRequest {
            request_header,
            subscription_ids,
            send_initial_values,
        })
    }
}
