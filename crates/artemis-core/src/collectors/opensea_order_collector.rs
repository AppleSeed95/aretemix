use crate::types::{Collector, CollectorStream};
use anyhow::Result;
use async_trait::async_trait;
use opensea_stream::{
    client,
    schema::{self, ItemListedData},
    subscribe_to, Collection, Network,
};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

/// A collector that listens for new orders on OpenSea, and generates a stream of
/// [events](OpenseaOrder) which contain the order.
#[derive(Default)]
pub struct OpenseaOrderCollector {
    api_key: String,
}

impl OpenseaOrderCollector {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

/// A new order event, containing the internal order.
#[derive(Debug, Clone)]
pub struct OpenseaOrder {
    pub listing: ItemListedData,
}

/// Implementation of the [Collector](Collector) trait for the [OpenseaOrderCollector](OpenseaOrderCollector).
