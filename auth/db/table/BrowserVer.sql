CREATE TABLE `authBrowserVer` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `authBrowserId` BIGINT UNSIGNED NOT NULL,
  `major` SMALLINT UNSIGNED NOT NULL,
  `minor` SMALLINT UNSIGNED NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `authBrowserId` (`authBrowserId`,`major`,`minor`)
);