use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use chrono::NaiveDate;
use hiq_fetch::{StockFetch, StockInfo};
use mongodb::{bson::doc, options::FindOptions, Client};
use tokio::sync::mpsc;

use crate::{
    store::{mongo::service::query_one, HiqCache, DATA_DEF_START_DATE, TAB_STOCK_MARGIN},
    syncer::{need_to_start, retry, AsyncFunc, Syncer},
    types::HiqSyncData,
    Error, Result,
};

use super::service::insert_many;

struct StockMarginAsyncFunc<'a> {
    fetch: Arc<dyn StockFetch>,
    code: &'a str,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
}

#[async_trait]
impl<'a> AsyncFunc for StockMarginAsyncFunc<'a> {
    async fn call(&self) -> Result<Option<HiqSyncData>> {
        let data = self
            .fetch
            .fetch_stock_margin(self.code, self.start, self.end)
            .await?;
        if data.is_empty() {
            Ok(None)
        } else {
            Ok(Some(HiqSyncData::StockMargin(data)))
        }
    }
}

pub(crate) struct StockMarginSyncer {
    fetch: Arc<dyn StockFetch>,
    cache: Arc<RwLock<HiqCache>>,
    client: Client,
    codes: Vec<StockInfo>,
    task_n: usize,
}

impl StockMarginSyncer {
    pub fn new(
        client: Client,
        fetch: Arc<dyn StockFetch>,
        cache: Arc<RwLock<HiqCache>>,
        codes: Vec<StockInfo>,
        task_n: usize,
    ) -> Self {
        Self {
            client,
            fetch,
            cache,
            codes,
            task_n,
        }
    }
}

#[async_trait]
impl Syncer for StockMarginSyncer {
    async fn fetch(&self, tx: mpsc::UnboundedSender<HiqSyncData>) -> Result<()> {
        for info in self.codes.iter() {
            log::info!(
                "start sync {}({}) {}, task#{}",
                info.name.as_str(),
                info.code.as_str(),
                TAB_STOCK_MARGIN,
                self.task_n
            );

            let bar: Option<hiq_fetch::StockMargin> = query_one(
                self.client.clone(),
                TAB_STOCK_MARGIN,
                doc! {"code": info.code.as_str()},
                FindOptions::builder()
                    .sort(doc! {"trade_date": -1})
                    .limit(1)
                    .build(),
            )
            .await?;

            let start = bar
                .map(|b| {
                    let cache = self.cache.read().unwrap();
                    Some(cache.next_trade_date(&(b.trade_date.date())))
                })
                .unwrap_or(Some(
                    NaiveDate::parse_from_str(DATA_DEF_START_DATE, "%Y-%m-%d").unwrap(),
                ));

            if !need_to_start(&start) {
                log::info!(
                    "{}({}) {} is the newest",
                    info.name.as_str(),
                    info.code.as_str(),
                    TAB_STOCK_MARGIN,
                );
                continue;
            }

            log::info!(
                "start sync {}({}) {}, start={:?}, end=None, task#{}",
                info.name.as_str(),
                info.code.as_str(),
                TAB_STOCK_MARGIN,
                &start,
                self.task_n
            );
            let func = StockMarginAsyncFunc {
                fetch: self.fetch.clone(),
                code: info.code.as_str(),
                start,
                end: None,
            };
            let data = retry(func).await?;
            if let Some(data) = data {
                tx.send(data).map_err(|e| {
                    log::error!("send data error {:?}", e);
                    Error::Custom(format!("send data error {:?}", e))
                })?;
            };
            log::info!(
                "end fetch {}({}) {}, start={:?}, end=None",
                info.name.as_str(),
                info.code.as_str(),
                TAB_STOCK_MARGIN,
                &start
            );
        }

        Ok(())
    }

    async fn save(&self, data: HiqSyncData) -> Result<()> {
        if let HiqSyncData::StockMargin(info) = data {
            let bar = info.get(0).unwrap();
            let len = info.len();
            log::info!(
                "start save {}({}) {}, size={}",
                bar.name.as_str(),
                bar.code.as_str(),
                TAB_STOCK_MARGIN,
                len
            );
            insert_many(self.client.clone(), TAB_STOCK_MARGIN, &info, false).await?;
            log::info!(
                "done save {}({}) {}, size={}",
                bar.name.as_str(),
                bar.code.as_str(),
                TAB_STOCK_MARGIN,
                len
            );
        }
        Ok(())
    }
}
