---
title: How to Tracing Databend
---

## Tracing In Databend

Databend using Rust's tracing ecosystem [tokio-tracing](https://github.com/tokio-rs/tracing) to do log and profile.

Databend default log level is `INFO`.

## Enable Tracing

```
LOG_LEVEL=DEBUG ./databend-query
```

If we want to track the execution of a query:

```sql
SET max_threads=1;
SELECT sum(number+1)+1 FROM numbers(10000) WHERE number>0 GROUP BY number%3;
```

## Tracing log

```
[2021-06-10T08:40:36Z DEBUG clickhouse_srv::cmd] Got packet Query(QueryRequest { query_id: "bac2b254-6245-4cae-910d-3e5e979c8b68", client_info: QueryClientInfo { query_kind: 1, initial_user: "", initial_query_id: "", initial_address: "0.0.0.0:0", interface: 1, os_user: "bohu", client_hostname: "thinkpad", client_name: "ClickHouse ", client_version_major: 21, client_version_minor: 4, client_version_patch: 6, client_revision: 54447, http_method: 0, http_user_agent: "", quota_key: "" }, stage: 2, compression: 1, query: "SELECT sum(number+1)+1 from numbers(10000) where number>0 group by number%3;" })
Jun 10 16:40:36.131 DEBUG ThreadId(16) databend_query::sql::plan_parser: query="SELECT sum(number+1)+1 from numbers(10000) where number>0 group by number%3;"
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] parsing expr
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] parsing expr
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] prefix: Identifier(Ident { value: "number", quote_style: None })
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] get_next_precedence() Plus
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] next precedence: 30
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] parsing expr
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] prefix: Value(Number("1", false))
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] get_next_precedence() RParen
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] next precedence: 0
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] get_next_precedence() RParen
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] next precedence: 0
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] prefix: Function(Function { name: ObjectName([Ident { value: "sum", quote_style: None }]), args: [Unnamed(BinaryOp { left: Identifier(Ident { value: "number", quote_style: None }), op: Plus, right: Value(Number("1", false)) })], over: None, distinct: false })
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] get_next_precedence() Plus
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] next precedence: 30
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] parsing expr
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] prefix: Value(Number("1", false))
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] get_next_precedence() Word(Word { value: "from", quote_style: None, keyword: FROM })
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] next precedence: 0
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] get_next_precedence() Word(Word { value: "from", quote_style: None, keyword: FROM })
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] next precedence: 0
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] parsing expr
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] prefix: Value(Number("10000", false))
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] get_next_precedence() RParen
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] next precedence: 0
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] parsing expr
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] prefix: Identifier(Ident { value: "number", quote_style: None })
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] get_next_precedence() Gt
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] next precedence: 20
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] parsing expr
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] prefix: Value(Number("0", false))
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] get_next_precedence() Word(Word { value: "group", quote_style: None, keyword: GROUP })
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] next precedence: 0
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] get_next_precedence() Word(Word { value: "group", quote_style: None, keyword: GROUP })
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] next precedence: 0
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] parsing expr
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] prefix: Identifier(Ident { value: "number", quote_style: None })
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] get_next_precedence() Mod
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] next precedence: 40
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] parsing expr
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] prefix: Value(Number("3", false))
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] get_next_precedence() SemiColon
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] next precedence: 0
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] get_next_precedence() SemiColon
[2021-06-10T08:40:36Z DEBUG sqlparser::parser] next precedence: 0
Jun 10 16:40:36.135  INFO ThreadId(16) sql_statement_to_plan: databend_query::sql::plan_parser: new
Jun 10 16:40:36.136  INFO ThreadId(16) sql_statement_to_plan: databend_query::sql::plan_parser: enter
Jun 10 16:40:36.136  INFO ThreadId(16) sql_statement_to_plan:select_to_plan: databend_query::sql::plan_parser: new
Jun 10 16:40:36.136  INFO ThreadId(16) sql_statement_to_plan:select_to_plan: databend_query::sql::plan_parser: enter
Jun 10 16:40:36.139  INFO ThreadId(16) sql_statement_to_plan:select_to_plan: databend_query::sql::plan_parser: exit
Jun 10 16:40:36.139  INFO ThreadId(16) sql_statement_to_plan:select_to_plan: databend_query::sql::plan_parser: close time.busy=2.65ms time.idle=457µs
Jun 10 16:40:36.139  INFO ThreadId(16) sql_statement_to_plan: databend_query::sql::plan_parser: exit
Jun 10 16:40:36.139  INFO ThreadId(16) sql_statement_to_plan: databend_query::sql::plan_parser: close time.busy=3.57ms time.idle=453µs
Jun 10 16:40:36.140  INFO ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::interpreters::interpreter_select: new
Jun 10 16:40:36.141  INFO ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::interpreters::interpreter_select: enter
Jun 10 16:40:36.141 DEBUG ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::optimizers::optimizer: Before ProjectionPushDown
Projection: (sum((number + 1)) + 1):UInt64
  Expression: (sum((number + 1)) + 1):UInt64 (Before Projection)
    AggregatorFinal: groupBy=[[(number % 3)]], aggr=[[sum((number + 1))]]
      AggregatorPartial: groupBy=[[(number % 3)]], aggr=[[sum((number + 1))]]
        Expression: (number % 3):UInt8, (number + 1):UInt64 (Before GroupBy)
          Filter: (number > 0)
            ReadDataSource: scan partitions: [1], scan schema: [number:UInt64], statistics: [read_rows: 10000, read_bytes: 80000]
Jun 10 16:40:36.142 DEBUG ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::optimizers::optimizer: After ProjectionPushDown
Projection: (sum((number + 1)) + 1):UInt64
  Expression: (sum((number + 1)) + 1):UInt64 (Before Projection)
    AggregatorFinal: groupBy=[[(number % 3)]], aggr=[[sum((number + 1))]]
      AggregatorPartial: groupBy=[[(number % 3)]], aggr=[[sum((number + 1))]]
        Expression: (number % 3):UInt8, (number + 1):UInt64 (Before GroupBy)
          Filter: (number > 0)
            ReadDataSource: scan partitions: [1], scan schema: [number:UInt64], statistics: [read_rows: 10000, read_bytes: 80000]
Jun 10 16:40:36.142 DEBUG ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::optimizers::optimizer: Before Scatters
Projection: (sum((number + 1)) + 1):UInt64
  Expression: (sum((number + 1)) + 1):UInt64 (Before Projection)
    AggregatorFinal: groupBy=[[(number % 3)]], aggr=[[sum((number + 1))]]
      AggregatorPartial: groupBy=[[(number % 3)]], aggr=[[sum((number + 1))]]
        Expression: (number % 3):UInt8, (number + 1):UInt64 (Before GroupBy)
          Filter: (number > 0)
            ReadDataSource: scan partitions: [1], scan schema: [number:UInt64], statistics: [read_rows: 10000, read_bytes: 80000]
Jun 10 16:40:36.143 DEBUG ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::optimizers::optimizer: After Scatters
Projection: (sum((number + 1)) + 1):UInt64
  Expression: (sum((number + 1)) + 1):UInt64 (Before Projection)
    AggregatorFinal: groupBy=[[(number % 3)]], aggr=[[sum((number + 1))]]
      AggregatorPartial: groupBy=[[(number % 3)]], aggr=[[sum((number + 1))]]
        Expression: (number % 3):UInt8, (number + 1):UInt64 (Before GroupBy)
          Filter: (number > 0)
            ReadDataSource: scan partitions: [1], scan schema: [number:UInt64], statistics: [read_rows: 10000, read_bytes: 80000]
Jun 10 16:40:36.143  INFO ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}:reschedule: databend_query::interpreters::plan_scheduler: new
Jun 10 16:40:36.143  INFO ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}:reschedule: databend_query::interpreters::plan_scheduler: enter
Jun 10 16:40:36.143  INFO ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}:reschedule: databend_query::interpreters::plan_scheduler: exit
Jun 10 16:40:36.143  INFO ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}:reschedule: databend_query::interpreters::plan_scheduler: close time.busy=145µs time.idle=264µs
Jun 10 16:40:36.144  INFO ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}:build: databend_query::pipelines::processors::pipeline_builder: new
Jun 10 16:40:36.144  INFO ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}:build: databend_query::pipelines::processors::pipeline_builder: enter
Jun 10 16:40:36.144 DEBUG ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}:build: databend_query::pipelines::processors::pipeline_builder: Received plan:
Projection: (sum((number + 1)) + 1):UInt64
  Expression: (sum((number + 1)) + 1):UInt64 (Before Projection)
    AggregatorFinal: groupBy=[[(number % 3)]], aggr=[[sum((number + 1))]]
      AggregatorPartial: groupBy=[[(number % 3)]], aggr=[[sum((number + 1))]]
        Expression: (number % 3):UInt8, (number + 1):UInt64 (Before GroupBy)
          Filter: (number > 0)
            ReadDataSource: scan partitions: [1], scan schema: [number:UInt64], statistics: [read_rows: 10000, read_bytes: 80000]
Jun 10 16:40:36.145 DEBUG ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}:build: databend_query::pipelines::processors::pipeline_builder: Pipeline:
ProjectionTransform × 1 processor
  ExpressionTransform × 1 processor
    GroupByFinalTransform × 1 processor
      GroupByPartialTransform × 1 processor
        ExpressionTransform × 1 processor
          FilterTransform × 1 processor
            SourceTransform × 1 processor
Jun 10 16:40:36.145  INFO ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}:build: databend_query::pipelines::processors::pipeline_builder: exit
Jun 10 16:40:36.145  INFO ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}:build: databend_query::pipelines::processors::pipeline_builder: close time.busy=1.07ms time.idle=215µs
Jun 10 16:40:36.145 DEBUG ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::pipelines::transforms::transform_projection: execute...
Jun 10 16:40:36.145 DEBUG ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::pipelines::transforms::transform_groupby_final: execute...
Jun 10 16:40:36.146 DEBUG ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::pipelines::transforms::transform_groupby_partial: execute...
Jun 10 16:40:36.146 DEBUG ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::pipelines::transforms::transform_source: execute, table:system.numbers, is_remote:false...
Jun 10 16:40:36.148 DEBUG ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::pipelines::transforms::transform_filter: execute...
Jun 10 16:40:36.148 DEBUG ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::pipelines::transforms::transform_expression_executor: (filter executor) execute, actions: [Input(ActionInput { name: "number", return_type: UInt64 }), Constant(ActionConstant { name: "0", value: 0 }), Function(ActionFunction { name: "(number > 0)", func_name: ">", return_type: Boolean, is_aggregated: false, arg_names: ["number", "0"], arg_types: [UInt64, UInt64], arg_fields: [] })]
Jun 10 16:40:36.150 DEBUG ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::pipelines::transforms::transform_filter: Filter cost: 1.678104ms
Jun 10 16:40:36.150 DEBUG ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::pipelines::transforms::transform_expression_executor: (expression executor) execute, actions: [Input(ActionInput { name: "number", return_type: UInt64 }), Constant(ActionConstant { name: "3", value: 3 }), Function(ActionFunction { name: "(number % 3)", func_name: "%", return_type: UInt64, is_aggregated: false, arg_names: ["number", "3"], arg_types: [UInt64, UInt64], arg_fields: [] }), Input(ActionInput { name: "number", return_type: UInt64 }), Constant(ActionConstant { name: "1", value: 1 }), Function(ActionFunction { name: "(number + 1)", func_name: "+", return_type: UInt64, is_aggregated: false, arg_names: ["number", "1"], arg_types: [UInt64, UInt64], arg_fields: [] })]
Jun 10 16:40:36.165 DEBUG ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::pipelines::transforms::transform_groupby_partial: Group by partial cost: 18.822193ms
Jun 10 16:40:36.166 DEBUG ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::pipelines::transforms::transform_groupby_final: Group by final cost: 20.170851ms
Jun 10 16:40:36.167  INFO ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::interpreters::interpreter_select: exit
Jun 10 16:40:36.167  INFO ThreadId(309) execute{ctx.id="1c651744-3e73-4b94-9df0-dc031b73c626"}: databend_query::interpreters::interpreter_select: close time.busy=26.1ms time.idle=592µs
Jun 10 16:40:36.167 DEBUG ThreadId(309) databend_query::pipelines::transforms::transform_expression_executor: (expression executor) execute, actions: [Input(ActionInput { name: "sum((number + 1))", return_type: UInt64 }), Constant(ActionConstant { name: "1", value: 1 }), Function(ActionFunction { name: "(sum((number + 1)) + 1)", func_name: "+", return_type: UInt64, is_aggregated: false, arg_names: ["sum((number + 1))", "1"], arg_types: [UInt64, UInt64], arg_fields: [] })]
Jun 10 16:40:36.168 DEBUG ThreadId(309) databend_query::pipelines::transforms::transform_expression_executor: (projection executor) execute, actions: [Input(ActionInput { name: "(sum((number + 1)) + 1)", return_type: UInt64 })]
Jun 10 16:40:36.168 DEBUG ThreadId(309) databend_query::pipelines::transforms::transform_projection: Projection cost: 241.864µs
```

## Distributed tracing with Jaeger

### Start Databend

```
RUST_LOG=DEBUG, DATABEND_JAEGER_AGENT_ENDPOINT=localhost:6831 ./databend-query
```

### Start jaeger

```
docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest
```

### Create table and init datas

```
CREATE TABLE t1(a INT);
INSERT INTO t1 VALUES(1);
INSERT INTO t1 SELECT * FROM t1;
```

### Jaeger show

Open http://127.0.0.1:16686/

![](https://datafuse-1253727613.cos.ap-hongkong.myqcloud.com/jaeger-tracing-show.png)

## Error Tracking and Performance Monitoring with Sentry

[Sentry](https://github.com/getsentry/sentry) is a developer-first error tracking and performance monitoring platform that helps developers see what actually matters, solve quicker, and learn continuously about their applications.

### Deploy Sentry

Use the SaaS service provided by [sentry.io](https://sentry.io), or deploy it yourself by following [Self-Hosted Sentry](https://develop.sentry.dev/self-hosted/).

<img src="/img/tracing/sentry-hosted.png"/>

### Create a Project

Create a Rust project in Sentry and get the DSN. The DSN in this example is `http://0c44e65426cb4f87ba059464c0740502@127.0.0.1:9000/5` 。

<img src="/img/tracing/sentry-get-dsn.png"/>

### Start Databend

The following two ways of starting are available.

- **Enable Error Tracking Only**

  This will only use the sentry-log feature, which will help us with error tracking.

  ```bash
  DATABEND_SENTRY_DSN="<your-sentry-dsn>" ./databend-query
  ```

  <img src="/img/tracing/sentry-error.png"/>

- **Also Enable Performance Monitoring**

  Setting `SENTRY_TRACES_SAMPLE_RATE` greater than `0.0` will allow sentry to perform trace sampling, which will help set up performance monitoring.

  ```bash
  DATABEND_SENTRY_DSN="<your-sentry-dsn>" SENTRY_TRACES_SAMPLE_RATE=1.0 LOG_LEVEL=DEBUG ./databend-query
  ```

  **Note:** Set `SENTRY_TRACES_SAMPLE_RATE` a to lower value in production.

  <img src="/img/tracing/sentry-performance.png"/>

## Explore and Diagnose with tokio-console

[tokio-console](https://github.com/tokio-rs/console) is a diagnostics and debugging tool for asynchronous Rust programs. Make sure you have the tool installed before you use it.

### Steps

1. Compile with specific `RUSTFLAGS` and features. We can use `--bin` to specify binary.

   ```shell
   RUSTFLAGS="--cfg tokio_unstable" cargo build --features tokio-console
   ```

2. Run `databend-meta` or/and `databend-query`, remembering to set the log level of the program you need to diagnose to `TRACE`.

   ```shell
   LOG_LEVEL=TRACE databend-query # for query
   ```

   or

   ```shell
   databend-meta --single --log-level=TRACE # for meta
   ```

3. Run `tokio-console`.

   ```shell
   tokio-console # default connection: http://127.0.0.1:6669
   ```

### Tips

Note that tokio-console only supports diagnostics for a single program at a time, so please ensure that **only one** program has a log level of `TRACE`. Otherwise, only the first program to occupy the port will be monitored.

If you need to diagnose multiple programs at the same time, consider using `TOKIO_CONSOLE_BIND` to assign different binds, for example:

```shell
TOKIO_CONSOLE_BIND=127.0.0.1:16667 LOG_LEVEL=TRACE target/debug/databend-query
tokio-console http://127.0.0.1:16667 # for query console, http://127.0.0.1:16667
databend-meta --single --log-level=TRACE
tokio-console # for meta console, http://127.0.0.1:6669
```

### Examples

**databend-query**

<img src="/img/tracing/query-console.png"/>

**databend-meta**

<img src="/img/tracing/meta-console.png"/>

**task in console**

<img src="/img/tracing/task-in-console.png"/>
