package main

import (
	"crypto/hmac"
	"crypto/sha1"
	"encoding/base32"
	"encoding/binary"
	"fmt"
	"strings"
	"time"
)

const dummyKey = "CQAPZTCGBILT3O4NOKXQVB4EZC7HZ77BQBREBDSQKUSVYT2SOHS64TYK325NZOVNP222CXSWCKM73LPGOPB3MLPQEOEPNBAXKAHYPKY"

func main() {
	key, err := decodeKey(dummyKey)
	if err != nil {
		panic(err)
	}
	ts := getTimestamp()
	hash := getHMAC(key, ts)
	totpCode := getTOTPCode(hash)
	fmt.Printf("Current TOTP code: %06d\n", totpCode)
}

func decodeKey(secretKey string) ([]byte, error) {
	b32Decoder := base32.StdEncoding.WithPadding(base32.NoPadding)

	// Base32 only uses uppercase letters so we remove possible spaces and convert
	// all the characters to uppercase.
	secretKey = strings.ToUpper(strings.TrimSpace(secretKey))

	keyBytes, err := b32Decoder.DecodeString(secretKey)
	if err != nil {
		return nil, fmt.Errorf("error decoding secret key: %w", err)
	}

	return keyBytes, nil
}

func getTimestamp() []byte {
	timeBytes := make([]byte, 8)
	now := time.Now().Unix()

	binary.BigEndian.PutUint64(timeBytes, uint64(now)/30)

	return timeBytes
}

func getHMAC(secretBytes, timeBytes []byte) []byte {
	hash := hmac.New(sha1.New, secretBytes)
	hash.Write(timeBytes)
	return hash.Sum(nil)
}

func getTOTPCode(hash []byte) uint32 {
	offset := hash[len(hash)-1] & 0x0F
	// 0x7FFFFFFF is the max int32 number that is represented byte
	// 0111 1111 1111 1111 1111 1111 1111 1111 in binary.
	truncated := binary.BigEndian.Uint32(hash[offset:]) & 0x7FFFFFFF

	// This will return a 6 digit number that contains the digits more to the right
	return truncated % 1_000_000
}
