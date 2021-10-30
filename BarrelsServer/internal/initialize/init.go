package initialize

import (
	"BarrelsServer/internal/handlers"
	"context"
)

func Initialize() {
	handlers.Server(context.Background(), "127.0.0.1:9999")
}
