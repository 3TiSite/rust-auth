CREATE TABLE `authUidMail` (
  `id` BIGINT UNSIGNED NOT NULL,`hostId` BIGINT UNSIGNED NOT NULL,`authMailId` BIGINT UNSIGNED NOT NULL,`ts` BIGINT UNSIGNED NOT NULL DEFAULT current_timestamp(),PRIMARY KEY (`id`),UNIQUE KEY `hostId` (`hostId`,`authMailId`)
);