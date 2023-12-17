CREATE TABLE `item_property_user` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `user_id` bigint unsigned NOT NULL,
    `item_id` bigint unsigned NOT NULL,
    `property_id` bigint unsigned NOT NULL,
    `quantity` bigint unsigned NOT NULL DEFAULT '1',
    `created_at` timestamp NULL DEFAULT NOW(),
    `updated_at` timestamp NULL DEFAULT NOW(),
    `deleted_at` timestamp NULL DEFAULT NOW(),
    PRIMARY KEY (`id`),
    UNIQUE KEY `item_id` (`item_id`, `user_id`, `property_id`),
    KEY `item_property_user_property_id_foreign` (`property_id`),
    KEY `item_user_user_id_foreign` (`user_id`),
    CONSTRAINT `item_property_user_item_id_foreign` FOREIGN KEY (`item_id`) REFERENCES `items` (`id`),
    CONSTRAINT `item_property_user_property_id_foreign` FOREIGN KEY (`property_id`) REFERENCES `item_properties` (`id`),
    CONSTRAINT `item_property_user_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;