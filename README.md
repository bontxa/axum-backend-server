# Axum-backend-server
making practice with Axum

This is my first attempt to create a webserver with Rust and Axum.<br>Please consider that is absolutely experimental and everything is made for fun and learning.<br>Lot of improvements has to be implemented, i'll make it as soon as I improve my skill.<br>If you want to try it, you have to be postgres installed, as it will fill a small database table.<br>You will need to create the table with this command:<br><br>```CREATE TABLE users (id SERIAL PRIMARY KEY, nome VARCHAR(25) NOT NULL, cognome VARCHAR(25) NOT NULL, email VARCHAR(50));```<br><br>You need to replace <value> filed in the .env file.
