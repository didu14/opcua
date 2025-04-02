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
    basic_types::*, encoding::*, node_id::NodeId, service_types::RelativePath, string::UAString,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeOperand {
    pub node_id: NodeId,
    pub alias: UAString,
    pub browse_path: RelativePath,
    pub attribute_id: u32,
    pub index_range: UAString,
}

impl BinaryEncoder<AttributeOperand> for AttributeOperand {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.node_id.byte_len();
        size += self.alias.byte_len();
        size += self.browse_path.byte_len();
        size += self.attribute_id.byte_len();
        size += self.index_range.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        size += self.alias.encode(stream)?;
        size += self.browse_path.encode(stream)?;
        size += self.attribute_id.encode(stream)?;
        size += self.index_range.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let node_id = NodeId::decode(stream, decoding_options)?;
        let alias = UAString::decode(stream, decoding_options)?;
        let browse_path = RelativePath::decode(stream, decoding_options)?;
        let attribute_id = u32::decode(stream, decoding_options)?;
        let index_range = UAString::decode(stream, decoding_options)?;
        Ok(AttributeOperand {
            node_id,
            alias,
            browse_path,
            attribute_id,
            index_range,
        })
    }
}
