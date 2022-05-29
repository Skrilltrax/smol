-- Add migration script here
ALTER TABLE urls
    RENAME short_url to slug;