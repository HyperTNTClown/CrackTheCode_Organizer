-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `users`
(
    `id`       int(11)      NOT NULL AUTO_INCREMENT,
    `name`     varchar(255) NOT NULL,
    `email`    varchar(255) NOT NULL,
    `password` varchar(255) NOT NULL,
    `salt`     varchar(255) NOT NULL,
    `admin`    bool         NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    UNIQUE KEY (`id`),
    UNIQUE KEY (`email`)
) AUTO_INCREMENT = 1;