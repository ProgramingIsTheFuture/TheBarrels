package handlers

import (
	"BarrelsServer/internal/models"
	"context"
	"encoding/json"
	"fmt"
	"net"
	"time"
)

const maxBufferSize = 1024

func Server(ctx context.Context, address string) (err error) {
	// Creates the packet listenner
	pc, err := net.ListenPacket("udp", address)
	if err != nil {
		return
	}

	defer pc.Close()

	// Channels to send data over the threads
	doneChan := make(chan error, 1)
	recv := make(chan models.Server, 1)

	go func() {
		// Receive data from the clients
		// Send the data receiv to the channels and this will be handled on the sendData method
		receivData(pc, recv, doneChan)
	}()

	go func() {
		// Send the data to all the clients
		sendData(pc, recv, doneChan)
	}()

	// just checking for error
	select {
	case err = <-doneChan:
		fmt.Println("err: ", err)
		return
	}
}

func receivData(pc net.PacketConn, recv chan models.Server, doneChan chan error) {
	// Server to store all the server information
	// Clients, IPs, etc...
	serv := models.Server{}

	buffer := make([]byte, maxBufferSize)
	for {
		// Read from the UDP connections
		n, addr, err := pc.ReadFrom(buffer)
		if err != nil {
			doneChan <- err
			return
		}

		var clientsend = models.ClientSend{}
		err = json.Unmarshal(buffer[:n], &clientsend)
		if err != nil {
			continue
		}

		// Check if exists
		// and if exists - update
		// else - add new one
		var client = models.Client{IP: addr.String(), Client: &clientsend}
		exists := serv.CheckByIP(addr.String(), true, &clientsend)
		if !exists {
			serv.InsertNewUser(&client)
		}

		// Passing the server to the sendData
		recv <- serv
	}
}

func sendData(pc net.PacketConn, recv chan models.Server, doneChan chan error) {
	var err error
	for {
		// Receive the Server
		serv := <-recv
		for _, i := range serv.Clients {

			for _, j := range serv.Clients {
				// Avoid sending the user data to himself
				if j.IP == i.IP {
					continue
				}

				addr, _ := net.ResolveUDPAddr("udp", j.IP)

				jsonClient, _ := json.Marshal(i.Client)

				deadline := time.Now().Add(time.Second)
				err = pc.SetWriteDeadline(deadline)
				if err != nil {
					doneChan <- err
					return
				}

				// Send the information to the client
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
