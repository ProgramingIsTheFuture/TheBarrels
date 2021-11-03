package database

import (
	"github.com/go-redis/redis/v8"
)

type DB struct {
	db *redis.Client
}

func Connect() *DB {
	db := redis.NewClient(&redis.Options{
		Addr:     "localhost:6379",
		Password: "", // no password set
		DB:       0,  // use default DB
	})

	return &DB{db: db}
}
