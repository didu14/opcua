// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock
//
// This file was autogenerated from Opc.Ua.Types.bsd by tools/schema/gen_types.js
//
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]
use std::io::{Read, Write};
#[allow(unused_imports)]
use crate::types::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    node_id::NodeId,
    string::UAString,
    expanded_node_id::ExpandedNodeId,
    service_types::enums::NodeClass,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AddReferencesItem {
    pub source_node_id: NodeId,
    pub reference_type_id: NodeId,
    pub is_forward: bool,
    pub target_server_uri: UAString,
    pub target_node_id: ExpandedNodeId,
    pub target_node_class: NodeClass,
}

impl MessageInfo for AddReferencesItem {
    fn object_id(&self) -> ObjectId {
        ObjectId::AddReferencesItem_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<AddReferencesItem> for AddReferencesItem {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.source_node_id.byte_len();
        size += self.reference_type_id.byte_len();
        size += self.is_forward.byte_len();
        size += self.target_server_uri.byte_len();
        size += self.target_node_id.byte_len();
        size += self.target_node_class.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.source_node_id.encode(stream)?;
        size += self.reference_type_id.encode(stream)?;
        size += self.is_forward.encode(stream)?;
        size += self.target_server_uri.encode(stream)?;
        size += self.target_node_id.encode(stream)?;
        size += self.target_node_class.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let source_node_id = NodeId::decode(stream, decoding_options)?;
        let reference_type_id = NodeId::decode(stream, decoding_options)?;
        let is_forward = bool::decode(stream, decoding_options)?;
        let target_server_uri = UAString::decode(stream, decoding_options)?;
        let target_node_id = ExpandedNodeId::decode(stream, decoding_options)?;
        let target_node_class = NodeClass::decode(stream, decoding_options)?;
        Ok(AddReferencesItem {
            source_node_id,
            reference_type_id,
            is_forward,
            target_server_uri,
            target_node_id,
            target_node_class,
        })
    }
}
