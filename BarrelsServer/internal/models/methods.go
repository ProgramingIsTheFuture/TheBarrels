package models

// Check if the IP is stored
// If the IP is stored this will update
// Return false if the IP is not stored
func (s *Server) CheckByIP(ip string, shouldUpdate bool, c *ClientSend) bool {
	for _, i := range s.Clients {
		if i.IP == ip {
			if shouldUpdate {
				i.Client = c
				return true
			}
			return true
		}
	}

	return false
}

// Append new user to the server
func (s *Server) InsertNewUser(c *Client) {
	s.Clients = append(s.Clients, c)
}
