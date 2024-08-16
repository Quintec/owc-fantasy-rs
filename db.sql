CREATE DATABASE IF NOT EXISTS owc_fantasy;
USE owc_fantasy;

-- Users table: Stores user information
CREATE TABLE Users (
    id INT UNIQUE NOT NULL,  -- Unique integer ID set at creation
    username VARCHAR(50) NOT NULL UNIQUE,
    avatar_url VARCHAR(200) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);

-- Players table: Stores player information
CREATE TABLE Players (
    id INT UNIQUE NOT NULL,  -- Unique integer ID set at creation
    username VARCHAR(50) NOT NULL UNIQUE,
    avatar_url VARCHAR(200) NOT NULL,
    country VARCHAR(5) NOT NULL,
    rank INT NOT NULL,
    eliminated BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);

-- Teams table: Stores team information
CREATE TABLE Teams (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    round ENUM('ro64', 'ro32', 'ro16', 'qf', 'sf', 'f', 'gf') NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE
);


-- TeamPlayers table: Junction table to link Teams and Players
CREATE TABLE TeamPlayers (
    team_id INT NOT NULL,
    player_id INT NOT NULL,
    PRIMARY KEY (team_id, player_id),
    FOREIGN KEY (team_id) REFERENCES Teams(id) ON DELETE CASCADE,
    FOREIGN KEY (player_id) REFERENCES Players(id) ON DELETE CASCADE
);

-- PlayerPrices table: Stores player prices for each round
CREATE TABLE PlayerPrices (
    player_id INT NOT NULL,
    round ENUM('ro64', 'ro32', 'ro16', 'qf', 'sf', 'f', 'gf') NOT NULL,
    price INT NOT NULL,
    PRIMARY KEY (player_id, round),
    FOREIGN KEY (player_id) REFERENCES Players(id) ON DELETE CASCADE
);
