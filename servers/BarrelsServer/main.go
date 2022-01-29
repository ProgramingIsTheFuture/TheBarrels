package main

import (
	"BarrelsServer/internal/initialize"
	"BarrelsServer/pkg/database"
)

func main() {
	db := database.Connect()

	initialize.Initialize(db)
}
