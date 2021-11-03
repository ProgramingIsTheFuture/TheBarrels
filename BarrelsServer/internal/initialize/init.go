package initialize

import (
	"BarrelsServer/internal/handlers"
	"BarrelsServer/internal/ports"
	"context"
)

func Initialize(db ports.Database) {
	handlers.Server(context.Background(), "127.0.0.1:9999", db)
}
