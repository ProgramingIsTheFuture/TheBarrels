package server

import (
	"fmt"
	"log"
	"net"

	"github.com/google/uuid"
)

type Users []User

type Server struct {
	Users               Users
	EventSenderReceiver Event
}

func NewServer() *Server {
	eventSender := make(Event, 20)
	return &Server{
		EventSenderReceiver: eventSender,
	}
}

func (s *Server) handleAllConns() {
	for {
		event := <-s.EventSenderReceiver
		if event.action == "sendMsg" {
			for _, user := range s.Users {
				user.Conn.Write([]byte(fmt.Sprintf("%s: %s\n", event.User.Username, event.Value)))
			}
		}

		if event.action == "userConnected" {
			s.Users = append(s.Users, event.User)
			log.Println(s.Users)
		}

		if event.action == "userDisconnected" {
			for i, user := range s.Users {
				if event.User.uuid == user.uuid {
					s.Users = append(s.Users[:i], s.Users[i+1:]...)
					s.EventSenderReceiver <- EventUser{action: "sendMsg", User: event.User, Value: "has disconnected!\n"}
					break
				}
			}

		}

	}
}

func (s *Server) Run() {
	log.Println("Server is starting")

	lstn, err := net.Listen("tcp", "0.0.0.0:9000")
	if err != nil {
		panic(err)
	}

	defer lstn.Close()

	go s.handleAllConns()
	for {
		conn, err := lstn.Accept()
		if err != nil {
			log.Fatalf("Err: %s\n", err.Error())
			continue
		}

		newUser := User{Conn: conn, uuid: uuid.New().String(), CanEnter: make(chan bool, 1), Authenticated: false}

		go newUser.Authenticate(s.EventSenderReceiver)
		go newUser.SendMsgsHandler(s.EventSenderReceiver)
	}
}
