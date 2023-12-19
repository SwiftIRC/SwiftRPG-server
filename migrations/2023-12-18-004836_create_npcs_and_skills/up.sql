CREATE TABLE `names` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    `species` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    `gender` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;
-- 
CREATE TABLE `occupations` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `name` varchar(100) COLLATE utf8mb4_unicode_ci NOT NULL,
    `description` varchar(1000) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
    `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
    `deleted_at` timestamp NULL DEFAULT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;
-- 
CREATE TABLE `npcs` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `first_name` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL,
    `last_name` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL,
    `species` enum('human', 'dwarf', 'elf') COLLATE utf8mb4_unicode_ci NOT NULL,
    `gender` enum('male', 'female', 'non-binary') COLLATE utf8mb4_unicode_ci NOT NULL,
    `occupation_id` bigint unsigned DEFAULT NULL,
    `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
    `deleted_at` timestamp NULL DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `npcs_occupation_id_foreign` (`occupation_id`),
    CONSTRAINT `npcs_occupation_id_foreign` FOREIGN KEY (`occupation_id`) REFERENCES `occupations` (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;
-- 
CREATE TABLE `skills` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `skills_name_unique` (`name`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;
-- 
CREATE TABLE `skill_user` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `skill_id` bigint unsigned NOT NULL,
    `user_id` bigint unsigned NOT NULL,
    `quantity` mediumint unsigned NOT NULL DEFAULT '0',
    PRIMARY KEY (`id`),
    KEY `skill_user_skill_id_foreign` (`skill_id`),
    KEY `skill_user_user_id_foreign` (`user_id`),
    CONSTRAINT `skill_user_skill_id_foreign` FOREIGN KEY (`skill_id`) REFERENCES `skills` (`id`),
    CONSTRAINT `skill_user_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;
-- 
CREATE TABLE `npc_skill` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `skill_id` bigint unsigned NOT NULL,
    `npc_id` bigint unsigned NOT NULL,
    `quantity` mediumint unsigned NOT NULL DEFAULT '0',
    PRIMARY KEY (`id`),
    KEY `npc_skill_skill_id_foreign` (`skill_id`),
    KEY `npc_skill_npc_id_foreign` (`npc_id`),
    CONSTRAINT `npc_skill_npc_id_foreign` FOREIGN KEY (`npc_id`) REFERENCES `npcs` (`id`),
    CONSTRAINT `npc_skill_skill_id_foreign` FOREIGN KEY (`skill_id`) REFERENCES `skills` (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;
-- 
CREATE TABLE `npc_tile` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `tile_id` bigint unsigned NOT NULL,
    `npc_id` bigint unsigned NOT NULL,
    `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
    `deleted_at` timestamp NULL DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `npc_tile_tile_id_foreign` (`tile_id`),
    KEY `npc_tile_npc_id_foreign` (`npc_id`),
    CONSTRAINT `npc_tile_npc_id_foreign` FOREIGN KEY (`npc_id`) REFERENCES `npcs` (`id`),
    CONSTRAINT `npc_tile_tile_id_foreign` FOREIGN KEY (`tile_id`) REFERENCES `tiles` (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;