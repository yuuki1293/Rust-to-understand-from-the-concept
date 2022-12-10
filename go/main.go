package main

import (
	"fmt"
	"os"
)

func main() {
	fp, err := os.Open("test.txt")
	if err == nil {
		buf := make([]byte, 1024)
		size_read, _ := fp.Read(buf)
		fmt.Fprintf(os.Stderr, "read size: %d\n", size_read)
	} else {
		fmt.Fprintf(os.Stderr, "Failed to open the file\n")
	}
}