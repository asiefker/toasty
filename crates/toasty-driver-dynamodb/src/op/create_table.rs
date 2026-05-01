use super::{
    AttributeDefinition, BillingMode, Connection, GlobalSecondaryIndex, Projection, ProjectionType,
    Result, Table, TypeExt, db, ddb_key_schema,
};
use toasty_core::schema::db::IndexScope;

impl Connection {
    pub(crate) async fn create_table(
        &mut self,
        schema: &db::Schema,
        table: &Table,
        reset: bool,
    ) -> Result<()> {
        if reset {
            let _ = self
                .client
                .delete_table()
                .table_name(&table.name)
                .send()
                .await;

            for index in &table.indices {
                if !index.primary_key && index.unique {
                    let _ = self
                        .client
                        .delete_table()
                        .table_name(&index.name)
                        .send()
                        .await;
                }
            }
        }

        let pk_index = &table.indices[table.primary_key.index.index];
        let partition_cols: Vec<&db::Column> = pk_index
            .columns
            .iter()
            .filter(|c| matches!(c.scope, IndexScope::Partition))
            .map(|c| table.column(c.column))
            .collect();
        let local_cols: Vec<&db::Column> = pk_index
            .columns
            .iter()
            .filter(|c| matches!(c.scope, IndexScope::Local))
            .map(|c| table.column(c.column))
            .collect();

        assert_eq!(
            partition_cols.len(),
            1,
            "table '{}' must have exactly one partition key",
            table.name
        );
        let partition_col = partition_cols[0];

        // When there are multiple Local-scoped PK columns the driver synthesises
        // a `__sk` string attribute at write time by concatenating them with `#`.
        let range_name: Option<String> = if local_cols.len() > 1 {
            Some("__sk".to_string())
        } else if local_cols.len() == 1 {
            Some(local_cols[0].name.clone())
        } else {
            None
        };

        // Collect attributes that need to be declared in the DynamoDB table schema.
        // Maps attribute name → DynamoDB type string.
        let mut defined_attributes: std::collections::HashMap<
            String,
            aws_sdk_dynamodb::types::ScalarAttributeType,
        > = std::collections::HashMap::new();
        defined_attributes.insert(partition_col.name.clone(), partition_col.ty.to_ddb_type());
        if let Some(ref rn) = range_name {
            if rn == "__sk" {
                // __sk is always a String type.
                defined_attributes.insert(
                    "__sk".to_string(),
                    aws_sdk_dynamodb::types::ScalarAttributeType::S,
                );
            } else {
                let range_col = local_cols[0];
                defined_attributes.insert(range_col.name.clone(), range_col.ty.to_ddb_type());
            }
        }

        let mut gsis = vec![];

        for index in &table.indices {
            if index.primary_key || index.unique {
                continue;
            }

            let gsi_partition_cols: Vec<&db::Column> = index
                .columns
                .iter()
                .filter(|ic| ic.scope.is_partition())
                .map(|ic| table.column(ic.column))
                .collect();

            let gsi_range_cols: Vec<&db::Column> = index
                .columns
                .iter()
                .filter(|ic| ic.scope.is_local())
                .map(|ic| table.column(ic.column))
                .collect();

            if gsi_partition_cols.is_empty() || gsi_partition_cols.len() > 4 {
                return Err(toasty_core::Error::invalid_schema(format!(
                    "GSI '{}' must have 1 to 4 partition (HASH) columns, got {}",
                    index.name,
                    gsi_partition_cols.len()
                )));
            }

            if gsi_range_cols.len() > 4 {
                return Err(toasty_core::Error::invalid_schema(format!(
                    "GSI '{}' must have at most 4 range (RANGE) columns, got {}",
                    index.name,
                    gsi_range_cols.len()
                )));
            }

            for col in &gsi_partition_cols {
                defined_attributes
                    .entry(col.name.clone())
                    .or_insert_with(|| col.ty.to_ddb_type());
            }
            for col in &gsi_range_cols {
                defined_attributes
                    .entry(col.name.clone())
                    .or_insert_with(|| col.ty.to_ddb_type());
            }

            let gsi_partition_name = gsi_partition_cols[0].name.as_str();
            let gsi_range_name = gsi_range_cols.first().map(|c| c.name.as_str());

            gsis.push(
                GlobalSecondaryIndex::builder()
                    .index_name(&index.name)
                    .set_key_schema(Some(ddb_key_schema(gsi_partition_name, gsi_range_name)))
                    .projection(
                        Projection::builder()
                            .projection_type(ProjectionType::All)
                            .build(),
                    )
                    .build()
                    .unwrap(),
            );
        }

        let attribute_definitions: Vec<_> = defined_attributes
            .iter()
            .map(|(name, ty)| {
                AttributeDefinition::builder()
                    .attribute_name(name)
                    .attribute_type(ty.clone())
                    .build()
                    .unwrap()
            })
            .collect();

        self.client
            .create_table()
            .table_name(&table.name)
            .set_attribute_definitions(Some(attribute_definitions))
            .set_key_schema(Some(ddb_key_schema(
                &partition_col.name,
                range_name.as_deref(),
            )))
            .set_global_secondary_indexes(if gsis.is_empty() { None } else { Some(gsis) })
            .billing_mode(BillingMode::PayPerRequest)
            .send()
            .await
            .map_err(toasty_core::Error::driver_operation_failed)?;

        // Create separate tables for each unique index.
        for index in table.indices.iter().filter(|i| !i.primary_key && i.unique) {
            assert_eq!(1, index.columns.len());

            let pk = schema.column(index.columns[0].column);

            self.client
                .create_table()
                .table_name(&index.name)
                .set_key_schema(Some(ddb_key_schema(&pk.name, None)))
                .attribute_definitions(
                    AttributeDefinition::builder()
                        .attribute_name(&pk.name)
                        .attribute_type(pk.ty.to_ddb_type())
                        .build()
                        .unwrap(),
                )
                .billing_mode(BillingMode::PayPerRequest)
                .send()
                .await
                .map_err(toasty_core::Error::driver_operation_failed)?;
        }

        Ok(())
    }
}
