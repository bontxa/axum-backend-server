﻿# Axum-backend-server
making practice with Axum

This is my first attempt to create a webserver with Rust and Axum.<br>Please consider that is absolutely experimental and everything is made for fun and learning.<br>Lot of improvements has to be implemented, i'll make it as soon as I improve my skill.<br>If you want to try it,do the following:<br>STEP 1<br>- this step has to be done only the first time you run this application, or every time after you do 'docker system prune -a'. In fact it it has to be done for freeing more than 2 useless gb.<br>cd in axum_project folder and run:<br>- docker build -t server .(don' t forget the '.' at the end)<br>- docker images<br>then remove every images excepted for 2 named 'server' and ‘debian’ with the command: docker rmi <replace-with-IMAGE_ID>, one at the time<br>STEP 2<br>- run:<br>- docker-compose up -d<br>that's all.<br>If you want to see your database status type:<br>psql -h localhost -d postgres -U postgres (type: postgres if it asks a password)<br>then type:<br> SELECT * FROM users;<br>type: exit for exiting postgres shell<br>test it with localhost:5000/index.html
