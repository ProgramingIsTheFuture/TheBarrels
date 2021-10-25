package main

import (
	"BarrelsServer/internal/udp"
	"BarrelsServer/pkg/database"
	"context"
)

func main() {
	database.Connect("Hello")
	udp.Server(context.Background(), "127.0.0.1:9999")
}
