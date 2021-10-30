package models

type Client struct {
	IP     string `json:"ip"`
	Client *ClientSend
}

type Player struct {
	ID        string  `json:"id"`
	X         float32 `json:"x"`
	Y         float32 `json:"y"`
	Direction int8    `json:"direction"`
	Moving    bool    `json:"moving"`
	CharCode  int8    `json:"char_code"`
	Username  string  `json:"username"`
}

type ClientSend struct {
	Action string `json:"action"`
	Player Player `json:"data"`
}

type Server struct {
	Clients []*Client
}
