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
    basic_types::*, date_time::DateTime, encoding::*, node_id::NodeId, node_ids::ObjectId,
    service_types::impls::MessageInfo, service_types::ApplicationDescription,
    service_types::ServiceCounterDataType, string::UAString,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct SessionDiagnosticsDataType {
    pub session_id: NodeId,
    pub session_name: UAString,
    pub client_description: ApplicationDescription,
    pub server_uri: UAString,
    pub endpoint_url: UAString,
    pub locale_ids: Option<Vec<UAString>>,
    pub actual_session_timeout: f64,
    pub max_response_message_size: u32,
    pub client_connection_time: DateTime,
    pub client_last_contact_time: DateTime,
    pub current_subscriptions_count: u32,
    pub current_monitored_items_count: u32,
    pub current_publish_requests_in_queue: u32,
    pub total_request_count: ServiceCounterDataType,
    pub unauthorized_request_count: u32,
    pub read_count: ServiceCounterDataType,
    pub history_read_count: ServiceCounterDataType,
    pub write_count: ServiceCounterDataType,
    pub history_update_count: ServiceCounterDataType,
    pub call_count: ServiceCounterDataType,
    pub create_monitored_items_count: ServiceCounterDataType,
    pub modify_monitored_items_count: ServiceCounterDataType,
    pub set_monitoring_mode_count: ServiceCounterDataType,
    pub set_triggering_count: ServiceCounterDataType,
    pub delete_monitored_items_count: ServiceCounterDataType,
    pub create_subscription_count: ServiceCounterDataType,
    pub modify_subscription_count: ServiceCounterDataType,
    pub set_publishing_mode_count: ServiceCounterDataType,
    pub publish_count: ServiceCounterDataType,
    pub republish_count: ServiceCounterDataType,
    pub transfer_subscriptions_count: ServiceCounterDataType,
    pub delete_subscriptions_count: ServiceCounterDataType,
    pub add_nodes_count: ServiceCounterDataType,
    pub add_references_count: ServiceCounterDataType,
    pub delete_nodes_count: ServiceCounterDataType,
    pub delete_references_count: ServiceCounterDataType,
    pub browse_count: ServiceCounterDataType,
    pub browse_next_count: ServiceCounterDataType,
    pub translate_browse_paths_to_node_ids_count: ServiceCounterDataType,
    pub query_first_count: ServiceCounterDataType,
    pub query_next_count: ServiceCounterDataType,
    pub register_nodes_count: ServiceCounterDataType,
    pub unregister_nodes_count: ServiceCounterDataType,
}

impl MessageInfo for SessionDiagnosticsDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::SessionDiagnosticsDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<SessionDiagnosticsDataType> for SessionDiagnosticsDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.session_id.byte_len();
        size += self.session_name.byte_len();
        size += self.client_description.byte_len();
        size += self.server_uri.byte_len();
        size += self.endpoint_url.byte_len();
        size += byte_len_array(&self.locale_ids);
        size += self.actual_session_timeout.byte_len();
        size += self.max_response_message_size.byte_len();
        size += self.client_connection_time.byte_len();
        size += self.client_last_contact_time.byte_len();
        size += self.current_subscriptions_count.byte_len();
        size += self.current_monitored_items_count.byte_len();
        size += self.current_publish_requests_in_queue.byte_len();
        size += self.total_request_count.byte_len();
        size += self.unauthorized_request_count.byte_len();
        size += self.read_count.byte_len();
        size += self.history_read_count.byte_len();
        size += self.write_count.byte_len();
        size += self.history_update_count.byte_len();
        size += self.call_count.byte_len();
        size += self.create_monitored_items_count.byte_len();
        size += self.modify_monitored_items_count.byte_len();
        size += self.set_monitoring_mode_count.byte_len();
        size += self.set_triggering_count.byte_len();
        size += self.delete_monitored_items_count.byte_len();
        size += self.create_subscription_count.byte_len();
        size += self.modify_subscription_count.byte_len();
        size += self.set_publishing_mode_count.byte_len();
        size += self.publish_count.byte_len();
        size += self.republish_count.byte_len();
        size += self.transfer_subscriptions_count.byte_len();
        size += self.delete_subscriptions_count.byte_len();
        size += self.add_nodes_count.byte_len();
        size += self.add_references_count.byte_len();
        size += self.delete_nodes_count.byte_len();
        size += self.delete_references_count.byte_len();
        size += self.browse_count.byte_len();
        size += self.browse_next_count.byte_len();
        size += self.translate_browse_paths_to_node_ids_count.byte_len();
        size += self.query_first_count.byte_len();
        size += self.query_next_count.byte_len();
        size += self.register_nodes_count.byte_len();
        size += self.unregister_nodes_count.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.session_id.encode(stream)?;
        size += self.session_name.encode(stream)?;
        size += self.client_description.encode(stream)?;
        size += self.server_uri.encode(stream)?;
        size += self.endpoint_url.encode(stream)?;
        size += write_array(stream, &self.locale_ids)?;
        size += self.actual_session_timeout.encode(stream)?;
        size += self.max_response_message_size.encode(stream)?;
        size += self.client_connection_time.encode(stream)?;
        size += self.client_last_contact_time.encode(stream)?;
        size += self.current_subscriptions_count.encode(stream)?;
        size += self.current_monitored_items_count.encode(stream)?;
        size += self.current_publish_requests_in_queue.encode(stream)?;
        size += self.total_request_count.encode(stream)?;
        size += self.unauthorized_request_count.encode(stream)?;
        size += self.read_count.encode(stream)?;
        size += self.history_read_count.encode(stream)?;
        size += self.write_count.encode(stream)?;
        size += self.history_update_count.encode(stream)?;
        size += self.call_count.encode(stream)?;
        size += self.create_monitored_items_count.encode(stream)?;
        size += self.modify_monitored_items_count.encode(stream)?;
        size += self.set_monitoring_mode_count.encode(stream)?;
        size += self.set_triggering_count.encode(stream)?;
        size += self.delete_monitored_items_count.encode(stream)?;
        size += self.create_subscription_count.encode(stream)?;
        size += self.modify_subscription_count.encode(stream)?;
        size += self.set_publishing_mode_count.encode(stream)?;
        size += self.publish_count.encode(stream)?;
        size += self.republish_count.encode(stream)?;
        size += self.transfer_subscriptions_count.encode(stream)?;
        size += self.delete_subscriptions_count.encode(stream)?;
        size += self.add_nodes_count.encode(stream)?;
        size += self.add_references_count.encode(stream)?;
        size += self.delete_nodes_count.encode(stream)?;
        size += self.delete_references_count.encode(stream)?;
        size += self.browse_count.encode(stream)?;
        size += self.browse_next_count.encode(stream)?;
        size += self
            .translate_browse_paths_to_node_ids_count
            .encode(stream)?;
        size += self.query_first_count.encode(stream)?;
        size += self.query_next_count.encode(stream)?;
        size += self.register_nodes_count.encode(stream)?;
        size += self.unregister_nodes_count.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let session_id = NodeId::decode(stream, decoding_options)?;
        let session_name = UAString::decode(stream, decoding_options)?;
        let client_description = ApplicationDescription::decode(stream, decoding_options)?;
        let server_uri = UAString::decode(stream, decoding_options)?;
        let endpoint_url = UAString::decode(stream, decoding_options)?;
        let locale_ids: Option<Vec<UAString>> = read_array(stream, decoding_options)?;
        let actual_session_timeout = f64::decode(stream, decoding_options)?;
        let max_response_message_size = u32::decode(stream, decoding_options)?;
        let client_connection_time = DateTime::decode(stream, decoding_options)?;
        let client_last_contact_time = DateTime::decode(stream, decoding_options)?;
        let current_subscriptions_count = u32::decode(stream, decoding_options)?;
        let current_monitored_items_count = u32::decode(stream, decoding_options)?;
        let current_publish_requests_in_queue = u32::decode(stream, decoding_options)?;
        let total_request_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let unauthorized_request_count = u32::decode(stream, decoding_options)?;
        let read_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let history_read_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let write_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let history_update_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let call_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let create_monitored_items_count =
            ServiceCounterDataType::decode(stream, decoding_options)?;
        let modify_monitored_items_count =
            ServiceCounterDataType::decode(stream, decoding_options)?;
        let set_monitoring_mode_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let set_triggering_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let delete_monitored_items_count =
            ServiceCounterDataType::decode(stream, decoding_options)?;
        let create_subscription_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let modify_subscription_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let set_publishing_mode_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let publish_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let republish_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let transfer_subscriptions_count =
            ServiceCounterDataType::decode(stream, decoding_options)?;
        let delete_subscriptions_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let add_nodes_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let add_references_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let delete_nodes_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let delete_references_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let browse_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let browse_next_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let translate_browse_paths_to_node_ids_count =
            ServiceCounterDataType::decode(stream, decoding_options)?;
        let query_first_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let query_next_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let register_nodes_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        let unregister_nodes_count = ServiceCounterDataType::decode(stream, decoding_options)?;
        Ok(SessionDiagnosticsDataType {
            session_id,
            session_name,
            client_description,
            server_uri,
            endpoint_url,
            locale_ids,
            actual_session_timeout,
            max_response_message_size,
            client_connection_time,
            client_last_contact_time,
            current_subscriptions_count,
            current_monitored_items_count,
            current_publish_requests_in_queue,
            total_request_count,
            unauthorized_request_count,
            read_count,
            history_read_count,
            write_count,
            history_update_count,
            call_count,
            create_monitored_items_count,
            modify_monitored_items_count,
            set_monitoring_mode_count,
            set_triggering_count,
            delete_monitored_items_count,
            create_subscription_count,
            modify_subscription_count,
            set_publishing_mode_count,
            publish_count,
            republish_count,
            transfer_subscriptions_count,
            delete_subscriptions_count,
            add_nodes_count,
            add_references_count,
            delete_nodes_count,
            delete_references_count,
            browse_count,
            browse_next_count,
            translate_browse_paths_to_node_ids_count,
            query_first_count,
            query_next_count,
            register_nodes_count,
            unregister_nodes_count,
        })
    }
}
