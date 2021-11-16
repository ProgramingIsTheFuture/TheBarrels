package handlers

import (
	"BarrelsServer/internal/models"
	"BarrelsServer/internal/ports"
	"context"
	"encoding/json"
	"fmt"
	"net"
	"strings"
	"time"

	"github.com/go-redis/redis/v8"
)

const maxBufferSize = 1024

type ServerStruct struct {
	DB ports.Database
	PC net.PacketConn
}

func Server(ctx context.Context, address string, db ports.Database) (err error) {
	// Creates the packet listenner
	pc, err := net.ListenPacket("udp", address)
	if err != nil {
		return
	}

	fmt.Println("Initializing the UDP server")

	defer pc.Close()

	// Channels to send data over the threads
	doneChan := make(chan error, 1)
	recv := make(chan models.Server, 1)
	clientsMsg := make(chan models.Client, 1)

	server := ServerStruct{PC: pc, DB: db}

	// Receive data from the clients
	// Send the data receiv to the channels and this will be handled on the sendData method
	go server.receivData(recv, clientsMsg, doneChan)

	go server.checkForDisconnect()

	// Send the data to all the clients
	go server.sendData(recv, clientsMsg, doneChan)

	// just checking for error
	select {
	case err = <-doneChan:
		fmt.Println("err: ", err)
		return
	}
}

func (s *ServerStruct) receivData(recv chan models.Server, clientMsg chan models.Client, doneChan chan error) {
	// Server to store all the server information
	// Clients, IPs, etc...
	// serv := models.Server{}

	buffer := make([]byte, maxBufferSize)
	for {
		// Read from the UDP connections
		n, addr, err := s.PC.ReadFrom(buffer)
		if err != nil {
			doneChan <- err
			return
		}

		var clientsend = models.ClientSend{}
		err = json.Unmarshal(buffer[:n], &clientsend)
		if err != nil {
			continue
		}

		clientsend.DateChanged = time.Now().Unix()

		// Check if exists
		// and if exists - update
		// else - add new one
		var client = models.Client{IP: addr.String(), Client: &clientsend}
		/*
			exists := serv.CheckByIP(addr.String(), true, &clientsend)
			if !exists {
				serv.InsertNewUser(&client)
			}
		*/

		// Passing the server to the sendData
		// recv <- serv

		fmt.Printf("Server - Data received from {%s}\n", clientsend.Player.Username)

		clientMsg <- client
	}
}

func (s *ServerStruct) sendData(recv chan models.Server, clientMsg chan models.Client, doneChan chan error) {
	// var err error
	for {
		// Receive the Server
		select {

		case serv := <-recv:
			go func() {
				for _, i := range serv.Clients {

					for _, j := range serv.Clients {
						// Avoid sending the user data to himself
						if j.IP == i.IP {
							continue
						}

						addr, _ := net.ResolveUDPAddr("udp", j.IP)

						jsonClient, _ := json.Marshal(i.Client)

						deadline := time.Now().Add(time.Second)
						err := s.PC.SetWriteDeadline(deadline)
						if err != nil {
							doneChan <- err
							return
						}

						// Send the information to the client
						_, err = s.PC.WriteTo(jsonClient, addr)
						if err != nil {
							doneChan <- err
							return
						}

					}
				}
			}()
		case client := <-clientMsg:
			go func() {
				strClient, err := stringify(client)
				if err != nil {
					return
				}

				var createNeeded = false
				c, err := s.DB.Get(fmt.Sprintf("ip-%s", client.IP))
				switch {
				case err == redis.Nil:
					createNeeded = true
					break
				case err != nil:
					return
				}

				if c != "" {
					if c == strClient {
						go func() {
							var c = client
							c.Client.DateChanged = time.Now().Unix()
							strC, err := stringify(c)
							if err != nil {
								return
							}
							err = s.DB.Set(c.IP, strC)
							if err != nil {
								return
							}
							return
						}()
						return
					}

					// c != strClient
					err := s.DB.Set(client.IP, strClient)
					if err != nil {
						return
					}
				}

				if createNeeded {
					fmt.Printf("Server - Creating new user {%s}\n", strClient)
					err = s.DB.Set(client.IP, strClient)
					if err != nil {
						return
					}
				}

				go s.SendAll(client.IP)
				return
			}()
		}
	}
}

func (s *ServerStruct) SendAll(ip string) {
	all, err := s.DB.GetAll()
	if err != nil {
		return
	}

	val, err := s.DB.Get(fmt.Sprintf("ip-%s", ip))
	if err != nil {
		return
	}

	var dataStruct models.Client
	err = toStruct(val, &dataStruct)
	if err != nil {
		return
	}

	data, err := stringify(dataStruct.Client)
	if err != nil {
		return
	}

	fmt.Printf("Server - Sending data {%s}\n", data)
	for _, key := range all {
		var k = strings.Split(key, "-")[1]
		if k == ip {
			continue
		}

		addr, err := net.ResolveUDPAddr("udp", k)
		if err != nil {
			continue
		}
		_, err = s.PC.WriteTo([]byte(data), addr)
		if err != nil {
			continue
		}
	}
}

func (s *ServerStruct) checkForDisconnect() {
	// all users will have a unix data
	// And we are going to check if passed more than 120 seconds
	// if so we will disconnect the user

	for {
	}
}
