package server

type Event chan EventUser

// Stores the action and a User that will get this action applied
// Events:
// UserConnected - Add user to the server user array
// UserDisconnected - Remove the user from the server array
// SendMessage - Send the message to all other users connected
type EventUser struct {
	action string
	User   User
	Value  string
}
