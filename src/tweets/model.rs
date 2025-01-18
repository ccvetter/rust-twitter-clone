use chrono::{DateTime, Utc};
use diesel::prelude::AsChangeset;
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db;
use crate::error_handler::CustomError;
use crate::likes::Like;
use crate::schema::tweets;

#[derive(Debug, Deserialize, Serialize, AsChangeset, Insertable, Queryable, Selectable)]
#[diesel(table_name = tweets)]
pub struct Tweet {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Tweets {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<Like>,
}

impl Tweets {
    pub fn create(message: String) -> Result<Tweet, CustomError> {
        let mut conn = db::connection()?;
        let tweet = Tweet::from(message);
        diesel::insert_into(tweets::table)
            .values(&tweet)
            .execute(&mut conn)
            .expect("Error saving tweet");

        Ok(tweet)
    }

     pub fn find(id: Uuid) -> Result<Tweet, CustomError> {
        let mut conn = db::connection()?;
        let tweet = tweets::table
            .filter(tweets::id.eq(id))
            .first(&mut conn)
            .expect("Error loading tweet");
        Ok(tweet)
    }

    pub fn find_all() -> Result<Vec<Tweet>, CustomError> {
        let mut conn = db::connection()?;
        let tweets = tweets::table
            .limit(50)
            .load::<Tweet>(&mut conn)
            .expect("Error loading tweets");
        Ok(tweets)
    }

    pub fn delete(id: Uuid) -> Result<usize, CustomError> {
        let mut conn = db::connection()?;
        let deleted_tweet = diesel::delete(tweets::table.filter(tweets::id.eq(id)))
            .execute(&mut conn)
            .expect("Error deleting tweet");
        Ok(deleted_tweet)
    }

}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
    pub message: Option<String>,
}

impl TweetRequest {
    pub fn to_tweet(&self) -> Option<Tweet> {
        match &self.message {
            Some(message) => Some(Tweet::from(message.to_string())),
            None => None,
        }
    }
}

impl Tweet {
    fn from(message: String) -> Tweet {
        Tweet {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            message: message
        }
    }
}
