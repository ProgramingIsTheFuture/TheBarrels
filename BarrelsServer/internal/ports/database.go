package ports

type Database interface {
	Get(string, interface{})
	GetAll()
}
