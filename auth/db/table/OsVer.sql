CREATE TABLE `authOsVer` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,`authOsId` INT UNSIGNED NOT NULL,`major` SMALLINT UNSIGNED NOT NULL,`minor` SMALLINT UNSIGNED NOT NULL,PRIMARY KEY (`id`),UNIQUE KEY `authOsId` (`authOsId`,`major`,`minor`)
);