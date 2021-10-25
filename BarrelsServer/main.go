package main

import (
	"context"
	"encoding/json"
	"fmt"
	"net"
	"time"
)

const maxBufferSize = 1024

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
	clients []*Client
}

// Check if the IP is stored
// If the IP is stored this will update
// Return false if the IP is not stored
func (s *Server) CheckByIP(ip string, shouldUpdate bool, c *ClientSend) bool {
	for _, i := range s.clients {
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
	s.clients = append(s.clients, c)
}

func server(ctx context.Context, address string) (err error) {
	pc, err := net.ListenPacket("udp", address)
	if err != nil {
		return
	}

	defer pc.Close()

	doneChan := make(chan error, 1)
	buffer := make([]byte, maxBufferSize)
	recv := make(chan Server, 1)

	go func() {
		serv := Server{}
		for {
			n, addr, err := pc.ReadFrom(buffer)
			if err != nil {
				doneChan <- err
				return
			}

			var clientsend = ClientSend{}
			err = json.Unmarshal(buffer[:n], &clientsend)
			if err != nil {
				continue
			}

			var client = Client{IP: addr.String(), Client: &clientsend}
			exists := serv.CheckByIP(addr.String(), true, &clientsend)
			if !exists {
				serv.InsertNewUser(&client)
			}
			if len(serv.clients) == 1 {
				fmt.Println("servClients: - ", serv.clients[0].Client.X, serv.clients[0].Client.Y)

			}
			recv <- serv
		}
	}()

	go func() {
		for {
			select {
			case serv := <-recv:
				// fmt.Println("servClients: - ", serv.clients)
				for _, i := range serv.clients {

					for _, j := range serv.clients {
						if j.IP == i.IP {
							continue
						}

						addr, _ := net.ResolveUDPAddr("udp", j.IP)
						fmt.Printf("Sending %s data - to %s", i.Client.Username, j.Client.Username)

						jsonClient, _ := json.Marshal(i.Client)

						deadline := time.Now().Add(time.Second)
						err = pc.SetWriteDeadline(deadline)
						if err != nil {
							doneChan <- err
							return
						}

						_, err := pc.WriteTo(jsonClient, addr)
						if err != nil {
							doneChan <- err
							return
						}

						fmt.Printf("packet-written: bytes=%s to=%s\n", string(jsonClient), addr.String())
					}
				}
			}
		}
	}()

	select {
	case <-ctx.Done():
		fmt.Println("cancelled")
		err = ctx.Err()
	case err = <-doneChan:
		fmt.Println("err: ", err)
	}

	return
}

func main() {
	server(context.Background(), "127.0.0.1:9999")
}
