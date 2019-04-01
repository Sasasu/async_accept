package main

import (
	"io"
	"net"
)

func main() {
	l, err := net.Listen("tcp", "127.0.0.1:10086")
	if err != nil {
		panic(err)
	}
	defer l.Close()

	for {
		conn, err := l.Accept()
		if err != nil {
			panic(err)
		}
		go handleConnection(conn)
	}
}

func handleConnection(conn net.Conn) {
	buf := make([]byte, 1024)
	len, err := conn.Read(buf)
	if err != nil {
		if err == io.EOF {
			conn.Close()
			return
		}
		panic(err)
	}
	if len == 1024 {
		panic("buffer too short")
	}
	resp := []byte("HTTP/1.1 204 No Content\r\nServer: go\r\n\r\n")
	conn.Write(resp)
	conn.Close()
}
