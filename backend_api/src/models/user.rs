use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::{
    error::errors::KekServerError, utils::deserialize_string_to_number,
    utils::serialize_id_to_string,
};

use super::ids::UserId;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(serialize_with = "serialize_id_to_string")]
    pub id: UserId,
    pub username: String,
    pub avatar: Option<String>,
}

impl User {
    pub async fn get_with_id(
        id: &UserId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, KekServerError> {
        match sqlx::query!(
            "
            SELECT * FROM users
            WHERE id = $1
            ",
            id.0 as i64
        )
        .fetch_optional(&mut *transaction)
        .await?
        {
            Some(r) => {
                return Ok(Some(Self {
                    id: UserId(r.id as u64),
                    username: r.username,
                    avatar: r.avatar,
                }));
            }
            None => return Ok(None),
        }
    }

    pub async fn insert_user(
        id: &UserId,
        username: &str,
        avatar: Option<&String>,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Self, KekServerError> {
        let r = sqlx::query!(
            "
            INSERT INTO users (id, username, avatar)
            VALUES ($1, $2, $3)
            RETURNING *
            ",
            id.0 as i64,
            username,
            avatar,
        )
        .fetch_one(&mut *transaction)
        .await?;
        return Ok(User {
            id: r.id.into(),
            username: r.username,
            avatar: r.avatar,
        });
    }
}

#[cfg(test)]
mod tests {
    use sqlx::Connection;
    use uuid::Uuid;

    use crate::{
        database::tests_db_helper::db_connection, models::ids::UserId,
        utils::test_utils::insert_user_test_util,
    };

    use super::User;

    impl User {
        pub fn get_test_user() -> User {
            return User {
                id: UserId(1),
                username: "user".to_owned(),
                avatar: None,
            };
        }
    }

    #[actix_web::test]
    async fn test_get_user_with_id() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();

        let user = insert_user_test_util(&mut transaction).await;

        let gotten_user = User::get_with_id(&user.id, &mut transaction)
            .await
            .unwrap()
            .unwrap();

        transaction.commit().await.unwrap();

        assert_eq!(gotten_user.id, user.id);
        assert_eq!(gotten_user.username, user.username);
        assert_eq!(gotten_user.avatar, user.avatar);
    }

    #[actix_web::test]
    async fn test_insert_user() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();
        let user_id = UserId(Uuid::new_v4().as_u128() as u64);

        let user = User::insert_user(&user_id, "test user", None, &mut transaction)
            .await
            .unwrap();

        transaction.commit().await.unwrap();

        assert_eq!(user.id, user_id);
    }
}
