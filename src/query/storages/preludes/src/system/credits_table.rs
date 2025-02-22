// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use common_datablocks::DataBlock;
use common_datavalues::prelude::*;
use common_exception::Result;
use common_meta_app::schema::TableIdent;
use common_meta_app::schema::TableInfo;
use common_meta_app::schema::TableMeta;

use crate::sessions::TableContext;
use crate::storages::system::table::SyncOneBlockSystemTable;
use crate::storages::system::table::SyncSystemTable;
use crate::storages::Table;

pub struct CreditsTable {
    table_info: TableInfo,
}

impl SyncSystemTable for CreditsTable {
    const NAME: &'static str = "system.credits";

    fn get_table_info(&self) -> &TableInfo {
        &self.table_info
    }

    fn get_full_data(&self, _: Arc<dyn TableContext>) -> Result<DataBlock> {
        let names: Vec<&[u8]> = env!("DATABEND_CREDITS_NAMES")
            .split_terminator(',')
            .map(|x| x.trim().as_bytes())
            .collect();
        let versions: Vec<&[u8]> = env!("DATABEND_CREDITS_VERSIONS")
            .split_terminator(',')
            .map(|x| x.trim().as_bytes())
            .collect();
        let licenses: Vec<&[u8]> = env!("DATABEND_CREDITS_LICENSES")
            .split_terminator(',')
            .map(|x| x.trim().as_bytes())
            .collect();
        Ok(DataBlock::create(self.table_info.schema(), vec![
            Series::from_data(names),
            Series::from_data(versions),
            Series::from_data(licenses),
        ]))
    }
}

impl CreditsTable {
    pub fn create(table_id: u64) -> Arc<dyn Table> {
        let schema = DataSchemaRefExt::create(vec![
            DataField::new("name", Vu8::to_data_type()),
            DataField::new("version", Vu8::to_data_type()),
            DataField::new("license", Vu8::to_data_type()),
        ]);

        let table_info = TableInfo {
            desc: "'system'.'credits'".to_string(),
            name: "credits".to_string(),
            ident: TableIdent::new(table_id, 0),
            meta: TableMeta {
                schema,
                engine: "SystemCredits".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        SyncOneBlockSystemTable::create(CreditsTable { table_info })
    }
}
