-- Your SQL goes here
CREATE TABLE `puzzles`
(
    `id`          int(11)       NOT NULL AUTO_INCREMENT,
    `name`        varchar(255) NOT NULL,
    `description` text         NOT NULL,
    `created`  datetime     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `modified`  datetime     NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
)