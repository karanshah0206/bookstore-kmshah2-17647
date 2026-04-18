-- Books Database Initialization Script
-- Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

CREATE DATABASE IF NOT EXISTS bookstore;
USE bookstore;
CREATE TABLE `books` (`isbn` varchar(255) NOT NULL, `title` varchar(255) NOT NULL, `author` varchar(255) NOT NULL, `description` varchar(255) NOT NULL, `genre` varchar(255) NOT NULL, `price` double NOT NULL, `quantity` bigint unsigned NOT NULL, `summary` longtext, PRIMARY KEY (`isbn`)) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
