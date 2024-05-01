-- Create the database
CREATE DATABASE IF NOT EXISTS CompanyDB;
USE CompanyDB;

-- Creating the Users table
CREATE TABLE IF NOT EXISTS Users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Creating the Groups table
CREATE TABLE IF NOT EXISTS Groups (
    id INT AUTO_INCREMENT PRIMARY KEY,
    group_name VARCHAR(255) NOT NULL,
    description TEXT
);

-- Creating the Teams table
CREATE TABLE IF NOT EXISTS Teams (
    id INT AUTO_INCREMENT PRIMARY KEY,
    team_name VARCHAR(255) NOT NULL,
    description TEXT
);

-- Creating the Transactions table
CREATE TABLE IF NOT EXISTS Transactions (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT,
    amount DECIMAL(10, 2),
    transaction_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES Users(id)
);

-- Creating a Users_Groups junction table for many-to-many relationship between Users and Groups
CREATE TABLE IF NOT EXISTS Users_Groups (
    user_id INT,
    group_id INT,
    PRIMARY KEY (user_id, group_id),
    FOREIGN KEY (user_id) REFERENCES Users(id),
    FOREIGN KEY (group_id) REFERENCES Groups(id)
);

-- Creating a Users_Teams junction table for many-to-many relationship between Users and Teams
CREATE TABLE IF NOT EXISTS Users_Teams (
    user_id INT,
    team_id INT,
    PRIMARY KEY (user_id, team_id),
    FOREIGN KEY (user_id) REFERENCES Users(id),
    FOREIGN KEY (team_id) REFERENCES Teams(id)
);

-- Seed data for Users
INSERT INTO Users (username, email) VALUES
('johndoe', 'john.doe@example.com'),
('janedoe', 'jane.doe@example.com');

-- Seed data for Groups
INSERT INTO Groups (group_name, description) VALUES
('Admins', 'System administrators group.'),
('Support', 'Customer support group.');

-- Seed data for Teams
INSERT INTO Teams (team_name, description) VALUES
('Development', 'Software development team.'),
('Sales', 'Sales and Marketing team.');

-- Seed data for Transactions
INSERT INTO Transactions (user_id, amount) VALUES
(1, 100.50),
(2, 200.75);

-- Seed data for Users_Groups
INSERT INTO Users_Groups (user_id, group_id) VALUES
(1, 1),
(2, 2);

-- Seed data for Users_Teams
INSERT INTO Users_Teams (user_id, team_id) VALUES
(1, 1),
(2, 2);
