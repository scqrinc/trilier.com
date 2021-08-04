package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strconv"
)

func main() {
	http.HandleFunc("/api/search", HandleSearch)
	fs := http.FileServer(http.Dir("./web/dist"))
	http.Handle("/", fs)
	fmt.Println("Server started at port 8000")
	http.ListenAndServe("localhost:8000", nil)
}

func HandleSearch(w http.ResponseWriter, r *http.Request) {
	fmt.Printf("<< method>> %s. ", r.Method)
	fmt.Printf("<< header >> %v\n<< body >> %v", r.Header, r.Body)
	// fmt.Fprintf(w, "%v", r)

	switch r.Header.Get("Content-Type") {
		case "application/x-www-form-urlencoded":
			fmt.Println("content type = form data")
			HandleSearchFormData(w, r)
		case "application/json":
			fmt.Println("content type = json")
			HandleSearchJson(w, r)
		default:
			fmt.Println("unknown content type")
	}
}

func HandleSearchFormData(w http.ResponseWriter, r *http.Request) {
	formData := r.ParseForm();
	fmt.Printf("form data = %v", formData)
}

func HandleSearchJson(w http.ResponseWriter, r *http.Request) {
	// To allocate slice for request body
	length, err := strconv.Atoi(r.Header.Get("Content-Length"))
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}
	// Read body data to parse json
	body := make([]byte, length)
	length, err = r.Body.Read(body)
	if err != nil && err != io.EOF {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}
	// parse json
	var jsonBody map[string]interface{}
	err = json.Unmarshal(body[:length], &jsonBody)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}
	fmt.Printf("%v\n", jsonBody)
}
