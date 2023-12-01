-- Your SQL goes here
CREATE TABLE `puzzle_set_refs`
(
    `id`            int(11) NOT NULL AUTO_INCREMENT,
    `puzzle_set_id` int(11) NOT NULL,
    `puzzle_id`     int(11) NOT NULL,
    PRIMARY KEY (`id`)
);