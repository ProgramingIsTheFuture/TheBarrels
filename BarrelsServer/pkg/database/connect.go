package database

import (
	"github.com/gocql/gocql"
	"github.com/scylladb/gocqlx/v2"
)

type DB struct {
	Session gocqlx.Session
}

func Connect(keyspace string) *DB {
	cluster := gocql.NewCluster("127.0.0.1:9042")
	cluster.Consistency = gocql.Quorum

	session, err := gocqlx.WrapSession(cluster.CreateSession())

	if err != nil {
		// Depends on this to work
		panic(err)
	}

	defer session.Close()

	return &DB{Session: session}
}
