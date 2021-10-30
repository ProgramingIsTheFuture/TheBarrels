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
	pc, err := net.ListenPacket("udp", address)
	if err != nil {
		return
	}

	defer pc.Close()

	doneChan := make(chan error, 1)
	buffer := make([]byte, maxBufferSize)
	recv := make(chan models.Server, 1)

	go func() {
		serv := models.Server{}
		for {
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

			var client = models.Client{IP: addr.String(), Client: &clientsend}
			exists := serv.CheckByIP(addr.String(), true, &clientsend)
			if !exists {
				serv.InsertNewUser(&client)
			}
			recv <- serv
		}
	}()

	go func() {
		for {
			select {
			case serv := <-recv:
				// fmt.Println("servClients: - ", serv.clients)
				for _, i := range serv.Clients {

					for _, j := range serv.Clients {
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
