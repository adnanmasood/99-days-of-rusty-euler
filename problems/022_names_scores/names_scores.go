// names_scores.go
// Go reference solution for Project Euler 22. Final answer redacted.
package main

import (
    "bufio"
    "fmt"
    "io/ioutil"
    "os"
    "sort"
    "strings"
    "unicode"
)

func alphaValue(s string) int64 {
    var v int64 = 0
    for _, r := range s {
        if unicode.IsLetter(r) {
            u := unicode.ToUpper(r)
            v += int64(u - 'A' + 1)
        }
    }
    return v
}

func parseNames(raw string) []string {
    parts := strings.Split(raw, ",")
    out := make([]string, 0, len(parts))
    for _, p := range parts {
        p = strings.TrimSpace(p)
        p = strings.Trim(p, "\"")
        if p != "" {
            out = append(out, p)
        }
    }
    return out
}

func main() {
    raw := ""
    if f, err := os.Open("names.txt"); err == nil {
        defer f.Close()
        data, _ := ioutil.ReadAll(bufio.NewReader(f))
        raw = string(data)
    } else {
        raw = "\"MARY\",\"PATRICIA\",\"LINDA\",\"BARBARA\",\"COLIN\""
    }

    names := parseNames(raw)
    sort.Strings(names)

    var total int64 = 0
    for i, name := range names {
        total += int64(i+1) * alphaValue(name)
    }

    // fmt.Println(total) // TODO: reveal locally
    fmt.Printf("Total of name scores = [redacted]; computed %d names\n", len(names))
}
