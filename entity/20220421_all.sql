-- Add migration script here
DROP TABLE IF EXISTS `user`;

CREATE TABLE `user` (
  `id` varchar(128) NOT NULL,
  `username` varchar(128) NOT NULL,
  `password` varchar(256) NOT NULL,
  `email` varchar(256),
  `phone` varchar(16),
  `is_actived` int(1) NOT NULL DEFAULT '1',
  `is_deleted` INT(1) NOT NULL DEFAULT '0',
  `is_admin` INT(1) NOT NULL DEFAULT '0',
  `last_logined_at` datetime,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
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
    CONSTRAINT `fk-category-users` FOREIGN KEY (`user_id`) REFERENCES `user`(`id`)
);


DROP TABLE IF EXISTS `medicinal`;

CREATE TABLE `medicinal` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `category_id` int(10) unsigned NOT NULL,
  `name` VARCHAR(512) NOT NULL,
  `batch_info` VARCHAR(512),
  `spec` VARCHAR(512),
  `count` VARCHAR(512),
  `validity` date NOT NULL,
  `is_deleted` INT(1) NOT NULL DEFAULT '0',
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` datetime COMMENT '更新时间',
  `notify_at` datetime COMMENT '发送告警时间',
  `user_id` VARCHAR(128) NOT NULL,
  PRIMARY key(`id`),
  UNIQUE KEY `idx_medicinal_id` (`id`, `user_id`),
  UNIQUE KEY `idx_category_medicinal_name` (`user_id`, `category_id`, `name`),
  CONSTRAINT `fk-medicinal-user` FOREIGN KEY (`user_id`) REFERENCES `user`(`id`),
  CONSTRAINT `fk-medicinal-category` FOREIGN KEY (`category_id`) REFERENCES `category`(`id`)
);