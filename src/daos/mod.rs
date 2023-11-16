use std::sync::Arc;

use sqlx::{Error, PgPool};

use crate::models::realm::{Realm, RealmCreate};

#[derive(Clone)]
pub struct RealmDao {
    pool: Arc<PgPool>,
}

impl RealmDao {
    pub fn new(pool: PgPool) -> Self {
        RealmDao {
            pool: Arc::new(pool),
        }
    }
    pub fn take_arc_pool(pool: Arc<PgPool>) -> Self {
        RealmDao { pool }
    }
    pub async fn list_all(&self) -> Result<Vec<Realm>, Error> {
        sqlx::query_as!(Realm, "SELECT * FROM astral_plane ORDER BY short_name")
            .fetch_all(&*self.pool)
            .await
    }
    pub async fn create(&self, realm: RealmCreate) -> Result<Realm, Error> {
        sqlx::query_as!(
            Realm,
            "INSERT INTO astral_plane (short_name, full_name, settlement_uri, landing_uri) VALUES ($1, $2, $3, $4) RETURNING *",
            realm.short_name,
            realm.full_name,
            realm.settlement_uri,
            realm.landing_uri
        )
        .fetch_one(&*self.pool)
        .await
    }
    pub async fn read(&self, realm_short_name: String) -> Result<Realm, Error> {
        let realm_short_name = realm_short_name.trim();
        sqlx::query_as!(
            Realm,
            "SELECT * FROM astral_plane WHERE short_name = $1",
            realm_short_name,
        )
        .fetch_one(&*self.pool)
        .await
    }
    pub async fn update(&self, realm_short_name: String, updated: Realm) -> Result<Realm, Error> {
        sqlx::query_as!(
        Realm,
            "UPDATE astral_plane SET (short_name, full_name, settlement_uri, landing_uri) = ($1, $2, $3, $4) WHERE short_name = $5 RETURNING *",
            updated.short_name,
            updated.full_name,
            updated.settlement_uri,
            updated.landing_uri,
            realm_short_name
        )
        .fetch_one(&*self.pool)
        .await
    }
    pub async fn increase_one(&self, realm_short_name: String) -> Result<Realm, Error> {
        sqlx::query_as!(
        Realm,
            "UPDATE astral_plane SET landing_count = landing_count + 1 WHERE short_name = $1 RETURNING *",
            realm_short_name
        )
        .fetch_one(&*self.pool)
        .await
    }
    pub async fn delete(&self, realm_short_name: String) -> Result<Realm, Error> {
        sqlx::query_as!(
            Realm,
            "DELETE FROM astral_plane WHERE short_name = $1 RETURNING *",
            realm_short_name
        )
        .fetch_one(&*self.pool)
        .await
    }
}
