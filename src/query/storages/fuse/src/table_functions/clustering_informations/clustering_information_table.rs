//  Copyright 2022 Datafuse Labs.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

use std::any::Any;
use std::sync::Arc;

use common_catalog::catalog::CATALOG_DEFAULT;
use common_datablocks::DataBlock;
use common_exception::Result;
use common_legacy_expression::LegacyExpression;
use common_legacy_planners::Extras;
use common_legacy_planners::Partitions;
use common_legacy_planners::ReadDataSourcePlan;
use common_legacy_planners::Statistics;
use common_meta_app::schema::TableIdent;
use common_meta_app::schema::TableInfo;
use common_meta_app::schema::TableMeta;

use super::clustering_information::ClusteringInformation;
use super::table_args::get_cluster_keys;
use super::table_args::parse_func_table_args;
use crate::pipelines::processors::port::OutputPort;
use crate::pipelines::processors::processor::ProcessorPtr;
use crate::pipelines::processors::AsyncSource;
use crate::pipelines::processors::AsyncSourcer;
use crate::pipelines::Pipe;
use crate::pipelines::Pipeline;
use crate::sessions::TableContext;
use crate::table_functions::string_literal;
use crate::table_functions::TableArgs;
use crate::table_functions::TableFunction;
use crate::FuseTable;
use crate::Table;

const FUSE_FUNC_CLUSTERING: &str = "clustering_information";

pub struct ClusteringInformationTable {
    table_info: TableInfo,
    arg_database_name: String,
    arg_table_name: String,
    // Todo(zhyass): support define cluster_keys.
    arg_cluster_keys: String,
}

impl ClusteringInformationTable {
    pub fn create(
        database_name: &str,
        table_func_name: &str,
        table_id: u64,
        table_args: TableArgs,
    ) -> Result<Arc<dyn TableFunction>> {
        let (arg_database_name, arg_table_name) = parse_func_table_args(&table_args)?;

        let engine = FUSE_FUNC_CLUSTERING.to_owned();

        let table_info = TableInfo {
            ident: TableIdent::new(table_id, 0),
            desc: format!("'{}'.'{}'", database_name, table_func_name),
            name: table_func_name.to_string(),
            meta: TableMeta {
                schema: ClusteringInformation::schema(),
                engine,
                ..Default::default()
            },
            ..Default::default()
        };

        Ok(Arc::new(Self {
            table_info,
            arg_database_name,
            arg_table_name,
            arg_cluster_keys: "".to_string(),
        }))
    }
}

#[async_trait::async_trait]
impl Table for ClusteringInformationTable {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_table_info(&self) -> &TableInfo {
        &self.table_info
    }

    async fn read_partitions(
        &self,
        _ctx: Arc<dyn TableContext>,
        _push_downs: Option<Extras>,
    ) -> Result<(Statistics, Partitions)> {
        Ok((Statistics::default(), vec![]))
    }

    fn table_args(&self) -> Option<Vec<LegacyExpression>> {
        Some(vec![
            string_literal(self.arg_database_name.as_str()),
            string_literal(self.arg_table_name.as_str()),
        ])
    }

    fn read_data(
        &self,
        ctx: Arc<dyn TableContext>,
        _: &ReadDataSourcePlan,
        pipeline: &mut Pipeline,
    ) -> Result<()> {
        let output = OutputPort::create();
        pipeline.add_pipe(Pipe::SimplePipe {
            inputs_port: vec![],
            outputs_port: vec![output.clone()],
            processors: vec![ClusteringInformationSource::create(
                ctx,
                output,
                self.arg_database_name.to_owned(),
                self.arg_table_name.to_owned(),
                self.arg_cluster_keys.to_owned(),
            )?],
        });

        Ok(())
    }
}

struct ClusteringInformationSource {
    finish: bool,
    ctx: Arc<dyn TableContext>,
    arg_database_name: String,
    arg_table_name: String,
    arg_cluster_keys: String,
}

impl ClusteringInformationSource {
    pub fn create(
        ctx: Arc<dyn TableContext>,
        output: Arc<OutputPort>,
        arg_database_name: String,
        arg_table_name: String,
        arg_cluster_keys: String,
    ) -> Result<ProcessorPtr> {
        AsyncSourcer::create(ctx.clone(), output, ClusteringInformationSource {
            ctx,
            finish: false,
            arg_table_name,
            arg_database_name,
            arg_cluster_keys,
        })
    }
}

#[async_trait::async_trait]
impl AsyncSource for ClusteringInformationSource {
    const NAME: &'static str = "clustering_information";

    #[async_trait::unboxed_simple]
    async fn generate(&mut self) -> Result<Option<DataBlock>> {
        if self.finish {
            return Ok(None);
        }

        self.finish = true;
        let tenant_id = self.ctx.get_tenant();
        let tbl = self
            .ctx
            .get_catalog(CATALOG_DEFAULT)?
            .get_table(
                tenant_id.as_str(),
                self.arg_database_name.as_str(),
                self.arg_table_name.as_str(),
            )
            .await?;

        let tbl = FuseTable::try_from_table(tbl.as_ref())?;
        let cluster_keys = get_cluster_keys(tbl, &self.arg_cluster_keys)?;

        Ok(Some(
            ClusteringInformation::new(self.ctx.clone(), tbl, cluster_keys)
                .get_clustering_info()
                .await?,
        ))
    }
}

impl TableFunction for ClusteringInformationTable {
    fn function_name(&self) -> &str {
        self.name()
    }

    fn as_table<'a>(self: Arc<Self>) -> Arc<dyn Table + 'a>
    where Self: 'a {
        self
    }
}
