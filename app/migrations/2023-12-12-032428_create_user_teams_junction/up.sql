-- Your SQL goes here
CREATE TABLE user_teams
(
    `user_id` INTEGER REFERENCES users (id),
    `team_id` INTEGER REFERENCES teams (id),
    PRIMARY KEY (user_id, team_id)
);