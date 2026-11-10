use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            r#"
            CREATE TABLE regions (
                id UUID PRIMARY KEY,
                code VARCHAR(64) NOT NULL UNIQUE,
                name VARCHAR(120) NOT NULL,
                location VARCHAR(160) NOT NULL,
                status VARCHAR(16) NOT NULL CHECK (status IN ('planned', 'enabled', 'disabled')),
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
            "#
        ).await?;
        manager.get_connection().execute_unprepared(
            r#"
            CREATE TABLE compute_offerings (
                id UUID PRIMARY KEY,
                region_id UUID NOT NULL REFERENCES regions(id) ON DELETE RESTRICT,
                code VARCHAR(64) NOT NULL,
                name VARCHAR(120) NOT NULL,
                capability_key VARCHAR(120) NOT NULL,
                vcpu_count INTEGER NOT NULL CHECK (vcpu_count > 0),
                memory_mib INTEGER NOT NULL CHECK (memory_mib >= 128),
                storage_gib INTEGER NOT NULL CHECK (storage_gib > 0),
                image_id VARCHAR(120) NOT NULL,
                status VARCHAR(16) NOT NULL CHECK (status IN ('unverified', 'enabled', 'disabled')),
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                CONSTRAINT uq_compute_offerings_region_code UNIQUE (region_id, code)
            )
            "#
        ).await?;
        manager.get_connection().execute_unprepared(
            "CREATE INDEX idx_regions_status_code ON regions (status, code)"
        ).await?;
        manager.get_connection().execute_unprepared(
            "CREATE INDEX idx_compute_offerings_region_status_code ON compute_offerings (region_id, status, code)"
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared("DROP TABLE IF EXISTS compute_offerings").await?;
        manager.get_connection().execute_unprepared("DROP TABLE IF EXISTS regions").await?;
        Ok(())
    }
}
