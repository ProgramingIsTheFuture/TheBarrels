---
sidebar_position: 2
---

# Game Server

Here goes a list of servers started or to implement:

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


### Global Chat

1. Convert message type from "string" to JSON
2. Add gRPC or other method to authenticate the user
