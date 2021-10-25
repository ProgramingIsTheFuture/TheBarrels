package models

type Client struct {
	IP     string `json:"ip"`
	Client *ClientSend
}

type ClientSend struct {
	ID       string  `json:"id"`
	X        float32 `json:"x"`
	Y        float32 `json:"y"`
	CharCode int8    `json:"char_code"`
	Username string  `json:"username"`
}

type Server struct {
	Clients []*Client
}
