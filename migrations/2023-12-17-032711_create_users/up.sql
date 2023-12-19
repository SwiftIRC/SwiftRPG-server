CREATE TABLE `users` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    `password` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    `is_admin` tinyint(1) NOT NULL DEFAULT '0',
    `remember_token` varchar(100) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
    `hitpoints` mediumint unsigned NOT NULL DEFAULT '100',
    `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
    `deleted_at` timestamp NULL DEFAULT NULL,
    `tile_id` bigint unsigned NOT NULL DEFAULT '1',
    `building_id` bigint unsigned DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `users_name_unique` (`name`),
    KEY `users_tile_id_foreign` (`tile_id`),
    KEY `users_building_id_foreign` (`building_id`),
    CONSTRAINT `users_building_id_foreign` FOREIGN KEY (`building_id`) REFERENCES `buildings` (`id`),
    CONSTRAINT `users_tile_id_foreign` FOREIGN KEY (`tile_id`) REFERENCES `tiles` (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;
-- 
ALTER TABLE `tiles`
ADD CONSTRAINT `tiles_discovered_by_foreign` FOREIGN KEY (`discovered_by`) REFERENCES `users` (`id`);
-- 
CREATE TABLE `item_property_user` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `user_id` bigint unsigned NOT NULL,
    `item_id` bigint unsigned NOT NULL,
    `property_id` bigint unsigned NOT NULL,
    `quantity` bigint unsigned NOT NULL DEFAULT '1',
    `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
    `deleted_at` timestamp NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `item_id` (`item_id`, `user_id`, `property_id`),
    KEY `item_property_user_property_id_foreign` (`property_id`),
    KEY `item_user_user_id_foreign` (`user_id`),
    CONSTRAINT `item_property_user_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`),
    CONSTRAINT `item_property_user_item_id_foreign` FOREIGN KEY (`item_id`) REFERENCES `items` (`id`),
    CONSTRAINT `item_property_user_property_id_foreign` FOREIGN KEY (`property_id`) REFERENCES `item_properties` (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;