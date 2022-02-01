package server

import (
	"io"
	"log"
	"net"
	"strings"

	"github.com/google/uuid"
)

/* User
* Stores the Connection
* Uuid from user
* Username from user
 */
type User struct {
	Conn          net.Conn
	uuid          string
	Username      string
	CanEnter      chan bool
	Authenticated bool
}

func (user *User) Authenticate(sender Event) {
	// First message show be a token
	p := make([]byte, 2048)
	n, err := user.Conn.Read(p)
	if err != nil {
		user.CanEnter <- false
		return
	}
	resp := string(p[:n])
	resp = strings.Trim(resp, " ")
	resp = strings.Trim(resp, "\n")
	if resp == "canlog" || resp == "login" {
		user.CanEnter <- true
		user.uuid = uuid.New().String()
		user.Username = resp
		sender <- EventUser{User: *user, action: "userConnected"}
		user.Conn.Write([]byte("Connected!!\n"))
		return
	}

	log.Println("Canot access! Leave!")
	user.Conn.Write([]byte("Leave, you are not welcome!\n"))
	user.CanEnter <- false
	return
}

func (user *User) SendMsgsHandler(sender Event) {
	if !(<-user.CanEnter) {
		user.Conn.Write([]byte("Not authenticated!\n"))
		user.Conn.Close()
		return
	}

	for {
		p := make([]byte, 2048)
		n, err := user.Conn.Read(p)
		if err != nil {
			if err == io.EOF {
				sender <- EventUser{action: "userDisconnected", User: *user, Value: ""}
				break
			}
		}

		sender <- EventUser{action: "sendMsg", User: *user, Value: string(p[:n])}
	}
}
