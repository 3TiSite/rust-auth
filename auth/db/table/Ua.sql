CREATE TABLE `authUa` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,`zone` SMALLINT DEFAULT NULL,`authOsVerId` BIGINT UNSIGNED DEFAULT NULL,`authBrowserVerId` BIGINT UNSIGNED DEFAULT NULL,`authLangId` BIGINT UNSIGNED DEFAULT NULL,`authHardwareId` BIGINT UNSIGNED NOT NULL,PRIMARY KEY (`id`),UNIQUE KEY `unqiue` (`zone`,`authOsVerId`,`authBrowserVerId`,`authLangId`,`authHardwareId`)
);