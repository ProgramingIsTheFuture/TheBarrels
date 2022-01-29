package handlers

import "encoding/json"

func stringify(v interface{}) (string, error) {
	b, err := json.Marshal(v)
	if err != nil {
		return "", err
	}

	return string(b), nil
}

func toStruct(v string, i interface{}) error {
	err := json.Unmarshal([]byte(v), i)
	if err != nil {
		return err
	}

	return nil
}
