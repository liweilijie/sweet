-- Add migration script here
DROP TABLE IF EXISTS `users`;

CREATE TABLE `users` (
  `id` varchar(128) NOT NULL,
  `username` varchar(128) NOT NULL,
  `password` varchar(256) NOT NULL,
  `email` varchar(256) NOT NULL DEFAULT '',
  `phone` varchar(16) NOT NULL DEFAULT '',
  `is_actived` int(1) NOT NULL DEFAULT '1',
  `is_deleted` INT(1) NOT NULL DEFAULT '0',
  `is_admin` INT(1) NOT NULL DEFAULT '0',
  `last_logined_at` datetime NOT NULL,
  `created_at` datetime NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_user_name` (`username`)
);


DROP TABLE IF EXISTS `category`;

CREATE TABLE `category` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(500) NOT NULL DEFAULT '',
    `user_id` VARCHAR(128) NOT NULL,
    PRIMARY KEY(`id`),
    UNIQUE KEY `idx_category_name` (`name`, `user_id`),
    CONSTRAINT `fk-category-users` FOREIGN KEY (`user_id`) REFERENCES `users`(`id`)
);


DROP TABLE IF EXISTS `medicinal`;

CREATE TABLE `medicinal` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `category_id` INT NOT NULL,
  `name` VARCHAR(512) NOT NULL,
  `batch_info` VARCHAR(512),
  `spec` VARCHAR(512),
  `count` VARCHAR(512),
  `validity` date NOT NULL,
  `is_deleted` INT(1) NOT NULL DEFAULT '0',
  `created_at` DATETIME NOT NULL,
  `updated_at` DATETIME NOT NULL,
  `notify_at` Timestamp NULL DEFAULT NULL,
  `user_id` VARCHAR(128) NOT NULL,
  PRIMARY key(`id`),
  UNIQUE KEY `idx_medicinal_id` (`id`, `user_id`),
  UNIQUE KEY `idx_category_medicinal_name` (`user_id`, `category_id`, `name`),
  CONSTRAINT `fk-medicinal-users` FOREIGN KEY (`user_id`) REFERENCES `users`(`id`)
);