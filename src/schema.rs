// @generated automatically by Diesel CLI.

diesel::table! {
    likes (id) {
        id -> Uuid,
        tweet_id -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    tweets (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        message -> Text,
    }
}

diesel::joinable!(likes -> tweets (tweet_id));

diesel::allow_tables_to_appear_in_same_query!(
    likes,
    tweets,
);
