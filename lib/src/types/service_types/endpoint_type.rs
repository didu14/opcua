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
    basic_types::*, encoding::*, node_ids::ObjectId, service_types::enums::MessageSecurityMode,
    service_types::impls::MessageInfo, string::UAString,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct EndpointType {
    pub endpoint_url: UAString,
    pub security_mode: MessageSecurityMode,
    pub security_policy_uri: UAString,
    pub transport_profile_uri: UAString,
}

impl MessageInfo for EndpointType {
    fn object_id(&self) -> ObjectId {
        ObjectId::EndpointType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<EndpointType> for EndpointType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.endpoint_url.byte_len();
        size += self.security_mode.byte_len();
        size += self.security_policy_uri.byte_len();
        size += self.transport_profile_uri.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.endpoint_url.encode(stream)?;
        size += self.security_mode.encode(stream)?;
        size += self.security_policy_uri.encode(stream)?;
        size += self.transport_profile_uri.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let endpoint_url = UAString::decode(stream, decoding_options)?;
        let security_mode = MessageSecurityMode::decode(stream, decoding_options)?;
        let security_policy_uri = UAString::decode(stream, decoding_options)?;
        let transport_profile_uri = UAString::decode(stream, decoding_options)?;
        Ok(EndpointType {
            endpoint_url,
            security_mode,
            security_policy_uri,
            transport_profile_uri,
        })
    }
}
