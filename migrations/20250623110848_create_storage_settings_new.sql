-- Add migration script here
CREATE TABLE storage_settings
(
    id                           SERIAL PRIMARY KEY,
    max_storage_days             INT NOT NULL,
    video_interval_minutes       INT NOT NULL,
    screenshots_interval_minutes INT NOT NULL
);