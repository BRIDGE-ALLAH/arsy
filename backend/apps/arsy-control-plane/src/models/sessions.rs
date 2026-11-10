use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, TransactionTrait,
};
use uuid::Uuid;

use super::_entities::auth_sessions;

/// Persist the session before its bearer token is returned to a client.
pub async fn create(
    db: &DatabaseConnection,
    user_id: i64,
    session_id: Uuid,
    expires_at: chrono::DateTime<chrono::FixedOffset>,
) -> Result<(), sea_orm::DbErr> {
    auth_sessions::ActiveModel {
        id: ActiveValue::Set(session_id),
        user_id: ActiveValue::Set(user_id),
        expires_at: ActiveValue::Set(expires_at),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(())
}

/// Session lookup fails closed for unknown, expired, revoked, or other-user sessions.
pub async fn is_active(
    db: &DatabaseConnection,
    user_id: i64,
    session_id: Uuid,
) -> Result<bool, sea_orm::DbErr> {
    let now = Utc::now().fixed_offset();
    let session = auth_sessions::Entity::find()
        .filter(auth_sessions::Column::Id.eq(session_id))
        .filter(auth_sessions::Column::UserId.eq(user_id))
        .filter(auth_sessions::Column::RevokedAt.is_null())
        .filter(auth_sessions::Column::ExpiresAt.gt(now))
        .one(db)
        .await?;
    Ok(session.is_some())
}

pub async fn revoke(
    db: &DatabaseConnection,
    user_id: i64,
    session_id: Uuid,
) -> Result<bool, sea_orm::DbErr> {
    let Some(model) = auth_sessions::Entity::find_by_id(session_id)
        .filter(auth_sessions::Column::UserId.eq(user_id))
        .filter(auth_sessions::Column::RevokedAt.is_null())
        .one(db)
        .await?
    else {
        return Ok(false);
    };

    let mut active: auth_sessions::ActiveModel = model.into();
    active.revoked_at = ActiveValue::Set(Some(Utc::now().fixed_offset()));
    active.update(db).await?;
    Ok(true)
}

/// Password reset revokes every still-active session atomically.
pub async fn revoke_all_for_user(
    db: &DatabaseConnection,
    user_id: i64,
) -> Result<u64, sea_orm::DbErr> {
    let transaction = db.begin().await?;
    let models = auth_sessions::Entity::find()
        .filter(auth_sessions::Column::UserId.eq(user_id))
        .filter(auth_sessions::Column::RevokedAt.is_null())
        .all(&transaction)
        .await?;

    let count = models.len() as u64;
    let revoked_at = Some(Utc::now().fixed_offset());
    for model in models {
        let mut active: auth_sessions::ActiveModel = model.into();
        active.revoked_at = ActiveValue::Set(revoked_at);
        active.update(&transaction).await?;
    }

    transaction.commit().await?;
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_identifiers_are_random_uuid_values() {
        let first = Uuid::new_v4();
        let second = Uuid::new_v4();
        assert_ne!(first, second);
    }
}
