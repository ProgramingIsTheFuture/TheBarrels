package database

import (
	"context"
	"fmt"
)

func (db *DB) Get(ip string) (string, error) {

	l := db.db.Get(context.Background(), ip)

	if l.Err() != nil {
		return "", l.Err()
	}

	return l.Val(), nil
}

func (db *DB) GetAll() ([]string, error) {

	keys, _, err := db.db.Scan(context.Background(), 0, "ip-*", 0).Result()
	if err != nil {
		return nil, err
	}

	return keys, nil
}

func (db *DB) Set(ip string, value string) error {

	l := db.db.Set(context.Background(), fmt.Sprintf("ip-%s", ip), value, 0)

	if l.Err() != nil {
		return l.Err()
	}

	return nil
}
