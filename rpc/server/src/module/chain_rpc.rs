// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::module::map_err;
use futures::future::TryFutureExt;
use starcoin_crypto::HashValue;
use starcoin_rpc_api::chain::ChainApi;
use starcoin_rpc_api::FutureResult;
use starcoin_traits::ChainAsyncService;
use starcoin_types::block::Block;
use starcoin_types::startup_info::ChainInfo;

pub struct ChainRpcImpl<S>
where
    S: ChainAsyncService + 'static,
{
    service: S,
}

impl<S> ChainRpcImpl<S>
where
    S: ChainAsyncService,
{
    pub fn new(service: S) -> Self {
        Self { service }
    }
}

impl<S> ChainApi for ChainRpcImpl<S>
where
    S: ChainAsyncService,
{
    fn head(&self) -> FutureResult<ChainInfo> {
        let fut = self.service.clone().master_head();
        Box::new(fut.map_err(|e: anyhow::Error| map_err(e.into())).compat())
    }

    fn get_block_by_hash(&self, hash: HashValue) -> FutureResult<Block> {
        let fut = self
            .service
            .clone()
            .get_block_by_hash(hash)
            .map_err(map_err);
        Box::new(fut.compat())
    }

    fn get_block_by_number(&self, number: u64) -> FutureResult<Block> {
        let fut = self
            .service
            .clone()
            .master_block_by_number(number)
            .map_err(map_err);
        Box::new(fut.compat())
    }

    fn get_blocks_by_number(&self, number: u64, count: u64) -> FutureResult<Vec<Block>> {
        let fut = self
            .service
            .clone()
            .master_blocks_by_number(number, count)
            .map_err(map_err);
        Box::new(fut.compat())
    }
}
