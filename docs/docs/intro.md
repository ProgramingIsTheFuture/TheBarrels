---
sidebar_position: 1
---

# TheBarrels 

This is an atepmt to build a good online game. 

[Demo](demo) page can be found here

## How this works

Here goes a list of servers started or to implement:

### Client using "Tauri" with React(typescript)
1. Let us communicate with friends and start the game ..
2. Choose and buy new characters

### The actual Game using Bevy:
- Will start if the player wants to play
- New Window
- Only Global Chat

### UDP Server using Golang (The Main server):
- Sends all important info from user to user
- Performance is required
- Uses redis as cache

### TCP Server using Golang:
- Will handle all global messages while in game

### Websocket Friends server:
- Will handle all real time messages for clients in private
- Add new friends
- Remove friends
- Real time update

### Status Server:
- Store all player status: 
  1. Coins
  2. Characters available
  3. Levels

