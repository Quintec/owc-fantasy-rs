-- Create a new database
CREATE DATABASE owc_fantasy;

-- Create a user and grant privileges
CREATE USER 'rustuser'@'localhost' IDENTIFIED BY 'password';

-- Grant all privileges on the new database to the new user
GRANT ALL PRIVILEGES ON owc_fantasy.* TO 'rustuser'@'localhost';

-- Flush the privileges to ensure that they are saved and available in the current session
FLUSH PRIVILEGES;

-- Exit the MySQL prompt
EXIT;
