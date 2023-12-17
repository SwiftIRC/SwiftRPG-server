CREATE TABLE `zones` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `name` varchar(100) COLLATE utf8mb4_unicode_ci NOT NULL,
    `description` varchar(1000) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
    `created_at` timestamp NULL DEFAULT NOW(),
    `updated_at` timestamp NULL DEFAULT NOW(),
    `deleted_at` timestamp NULL DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `zones_name_unique` (`name`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;
-- 
CREATE TABLE `buildings` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `name` varchar(100) COLLATE utf8mb4_unicode_ci NOT NULL,
    `description` varchar(1000) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
    `zone_id` bigint unsigned NOT NULL,
    `created_at` timestamp NULL DEFAULT NOW(),
    `updated_at` timestamp NULL DEFAULT NOW(),
    `deleted_at` timestamp NULL DEFAULT NULL,
    PRIMARY KEY (`id`),
    CONSTRAINT `buildings_zone_id_foreign` FOREIGN KEY (`zone_id`) REFERENCES `zones` (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;
-- 
CREATE TABLE `tiles` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `discovered_by` bigint unsigned DEFAULT NULL,
    `discovered_at` timestamp NULL DEFAULT NULL,
    `terrain_id` bigint unsigned NOT NULL,
    `psuedo_id` varchar(100) COLLATE utf8mb4_unicode_ci NOT NULL,
    `x` bigint NOT NULL,
    `y` bigint NOT NULL,
    `max_trees` smallint unsigned NOT NULL DEFAULT '0',
    `available_trees` smallint unsigned NOT NULL DEFAULT '0',
    `max_ore` smallint unsigned NOT NULL DEFAULT '0',
    `available_ore` smallint unsigned NOT NULL DEFAULT '0',
    `last_disturbed` timestamp NULL DEFAULT NULL,
    `created_at` timestamp NULL DEFAULT NULL,
    `updated_at` timestamp NULL DEFAULT NULL,
    `deleted_at` timestamp NULL DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `tiles_psuedo_id_unique` (`psuedo_id`)
) ENGINE = InnoDB AUTO_INCREMENT = 2 DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci