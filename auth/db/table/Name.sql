CREATE TABLE `authName` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,`val` VARBINARY(255) NOT NULL,PRIMARY KEY (`id`),UNIQUE KEY `val` (`val`)
);