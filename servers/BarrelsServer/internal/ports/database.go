package ports

type Database interface {
	Get(string) (string, error)
	GetAll() ([]string, error)
	Set(string, string) error
}
