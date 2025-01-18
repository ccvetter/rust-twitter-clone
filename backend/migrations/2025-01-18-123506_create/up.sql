ALTER TABLE likes 
    ADD CONSTRAINT fk_tweets_likes
    FOREIGN KEY (tweet_id)
    REFERENCES tweets (id);
    