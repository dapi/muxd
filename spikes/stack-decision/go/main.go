package main

import (
	"bufio"
	"encoding/json"
	"errors"
	"fmt"
	"net"
	"os"
	"os/exec"
	"syscall"
)

type request struct {
	Command string   `json:"command"`
	Args    []string `json:"args"`
	Cwd     string   `json:"cwd"`
}

type response struct {
	OK       bool   `json:"ok"`
	ExitCode int    `json:"exit_code"`
	Stdout   string `json:"stdout"`
	Stderr   string `json:"stderr"`
	Error    string `json:"error,omitempty"`
}

func main() {
	if len(os.Args) != 2 {
		fmt.Fprintf(os.Stderr, "usage: %s <socket-path>\n", os.Args[0])
		os.Exit(2)
	}

	socketPath := os.Args[1]
	_ = os.Remove(socketPath)

	listener, err := net.Listen("unix", socketPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "listen: %v\n", err)
		os.Exit(1)
	}
	defer os.Remove(socketPath)
	defer listener.Close()

	conn, err := listener.Accept()
	if err != nil {
		fmt.Fprintf(os.Stderr, "accept: %v\n", err)
		os.Exit(1)
	}
	defer conn.Close()

	req, err := readRequest(conn)
	if err != nil {
		writeResponse(conn, response{OK: false, ExitCode: 1, Error: err.Error()})
		os.Exit(1)
	}

	resp := run(req)
	if err := writeResponse(conn, resp); err != nil {
		fmt.Fprintf(os.Stderr, "write response: %v\n", err)
		os.Exit(1)
	}
}

func readRequest(conn net.Conn) (request, error) {
	line, err := bufio.NewReader(conn).ReadBytes('\n')
	if err != nil {
		return request{}, err
	}

	var req request
	if err := json.Unmarshal(line, &req); err != nil {
		return request{}, fmt.Errorf("decode request: %w", err)
	}
	if req.Command == "" {
		return request{}, errors.New("command is required")
	}
	return req, nil
}

func run(req request) response {
	cmd := exec.Command(req.Command, req.Args...)
	if req.Cwd != "" {
		cmd.Dir = req.Cwd
	}

	out, err := cmd.CombinedOutput()
	resp := response{OK: err == nil, ExitCode: 0, Stdout: string(out), Stderr: ""}
	if err == nil {
		return resp
	}

	var exitErr *exec.ExitError
	if errors.As(err, &exitErr) {
		resp.ExitCode = exitErr.ExitCode()
		if _, ok := exitErr.Sys().(syscall.WaitStatus); ok {
			resp.Error = err.Error()
		} else {
			resp.Error = err.Error()
		}
		return resp
	}

	resp.ExitCode = 1
	resp.Error = err.Error()
	return resp
}

func writeResponse(conn net.Conn, resp response) error {
	encoded, err := json.Marshal(resp)
	if err != nil {
		return err
	}
	_, err = conn.Write(append(encoded, '\n'))
	return err
}
