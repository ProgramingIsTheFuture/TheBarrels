package main

import (
	"GlobalChat/server"
)

func main() {
	s := server.NewServer()
	s.Run()
}
