use crate::endpoint_store::{EndpointStore, StoreError, ApiGroup, ApiGroupWithEndpoints};
use crate::endpoint_store::db_helpers::ResultExt;

/// Gets the default API groups from the database
pub(crate) async fn get_default_api_groups(
    store: &EndpointStore,
) -> Result<Vec<ApiGroupWithEndpoints>, StoreError> {
    tracing::info!("Fetching default API groups from database");
    
    // First get all default groups in a single transaction scope
    let groups: Vec<ApiGroup> = {
        let mut conn = store.get_conn().await?;
        let tx = conn.transaction().to_store_error()?;

        // Check if there are any default groups
        let default_count: i64 = tx
            .query_row(
                "SELECT COUNT(*) FROM api_groups WHERE is_default = true",
                [],
                |row| row.get(0),
            )
            .to_store_error()?;

        tracing::info!(
            count = default_count,
            "Found default API groups in database"
        );

        if default_count == 0 {
            tracing::warn!("No default API groups found in database");
            tx.commit().to_store_error()?;
            return Ok(Vec::new());
        }

        // Get all default groups
        let mut stmt = tx
            .prepare("SELECT id, name, description, base FROM api_groups WHERE is_default = true")
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to prepare statement for fetching default groups");
                StoreError::Database(e.to_string())
            })?;

        let groups_iter = stmt.query_map([], |row| {
            Ok(ApiGroup {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                base: row.get(3)?,
            })
        })
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to query default groups");
            StoreError::Database(e.to_string())
        })?;

        let groups = groups_iter.collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to process group result");
                StoreError::Database(e.to_string())
            })?;

        // Commit transaction before leaving this scope
        tx.commit().to_store_error()?;
        
        groups
    }; // Transaction and connection are dropped here

    // Now fetch endpoints for each group with separate connections
    let mut result = Vec::new();
    for group in groups {
        tracing::debug!(
            group_id = %group.id,
            group_name = %group.name,
            "Processing default group"
        );
        
        match store.get_endpoints_by_group_id(&group.id).await {
            Ok(endpoints) => {
                tracing::debug!(
                    group_id = %group.id,
                    endpoint_count = endpoints.len(),
                    "Retrieved endpoints for group"
                );
                result.push(ApiGroupWithEndpoints { group, endpoints });
            }
            Err(e) => {
                tracing::error!(
                    error = %e,
                    group_id = %group.id,
                    "Failed to get endpoints for group"
                );
                return Err(e);
            }
        }
    }

    tracing::info!(
        group_count = result.len(),
        "Successfully retrieved default API groups"
    );

    // Log details of each group for debugging
    for (i, group_with_endpoints) in result.iter().enumerate() {
        tracing::debug!(
            index = i,
            group_id = %group_with_endpoints.group.id,
            group_name = %group_with_endpoints.group.name,
            endpoint_count = group_with_endpoints.endpoints.len(),
            "Default group details"
        );
    }

    Ok(result)
}
