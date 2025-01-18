use chrono::{DateTime, Utc};
use diesel::prelude::Identifiable;
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db;
use crate::error_handler::CustomError;
use crate::schema::likes;

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable, Identifiable)]
#[diesel(table_name = likes)]
pub struct Like {
    pub id: Uuid,
    pub tweet_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Likes {
    pub id: Uuid,
    pub tweet_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl Likes {
    pub fn create(tweet_id: Uuid) -> Result<Like, CustomError> {
        let mut conn = db::connection()?;
        let like = Like::from(tweet_id);
        diesel::insert_into(likes::table)
            .values(&like)
            .execute(&mut conn)
            .expect("Error saving tweet");
        Ok(like)
    }

    pub fn find_all(tweet_id: Uuid) -> Result<Vec<Likes>, CustomError> {
        let mut conn = db::connection()?;
        
        let likes = likes::table
            .filter(likes::tweet_id.eq(tweet_id))
            .load::<Likes>(&mut conn)
            .expect("Error loading likes");
        Ok(likes)
    }

    pub fn delete(tweet_id: Uuid) -> Result<usize, CustomError> {
        let mut conn = db::connection()?;
        let like_to_delete = likes::table
            .filter(likes::tweet_id.eq(tweet_id))
            .order_by(likes::created_at.desc())
            .first::<Likes>(&mut conn)
            .expect("Error finding like to delete");

        let res = diesel::delete(likes::table.filter(likes::id.eq(like_to_delete.id)))
            .execute(&mut conn)
            .expect("Error deleting like");
        Ok(res)
    }
}

impl Like {
    fn from(tweet_id: Uuid) -> Like {
        Like {
            id: Uuid::new_v4(),
            tweet_id,
            created_at: Utc::now()
        }
    }
}
