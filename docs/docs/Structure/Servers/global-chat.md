---
sidebar_position: 3
---

# Global Chat

The core logic is working!

This server will depend on [OAuth server](auth-server)

After the user connects, the first message they sent should be a valid token 
otherwise they will be imediatly disconnected (Not authenticated).

After they server validate the user token, they are connected and can start sending messages.

