# TOTP Client

The idea is to implement a TOTP client that generates tokens for 2FA 
authentication.

## External Support Tools

- [TOTP Generator and Checker](https://www.verifyr.com/en/otp/check/) - Can be 
used to generate the secret key and check if the code is valid.

## Main Topics Covered

- Base32 Encoding
- HMAC
- Bitwise Operations
- Little and Big Endian

## Implementations

### Golang

Go standard library provides all the necessary tools to make the implementation.

### Rust


## RFC 6238 - TOTP Algorithm

### Initialization

A shared secret key is securely generated and shared between the server and the 
user's device.
Typically, a 16-32 character base32-encoded string is used as the secret key.
Time Interval (Time Step Size):

A fixed time interval (usually 30 seconds) is chosen as the period during which 
a TOTP code is valid.
All TOTP codes generated within the same time interval will be the same.

### Timestamp

The current time, usually represented as the number of seconds since the Unix 
epoch (January 1, 1970), is divided by the time interval to obtain a timestamp 
value.

### HMAC-SHA1 Calculation

Using the shared secret key and the timestamp, an HMAC-SHA1 hash is computed.
The HMAC input is typically the timestamp value converted into an 8-byte binary 
array.

### Dynamic Truncation

The 20-byte HMAC-SHA1 hash is dynamically truncated to a 4-byte binary string.
The dynamic truncation process involves extracting a 4-byte integer from the 
hash based on certain rules.

### OTP Generation

The 4-byte binary string from the previous step is used to generate a 6 or 
8-digit OTP.
The binary string is converted to an integer.
A modulo operation is performed to obtain a smaller integer that is then 
formatted as a fixed-length OTP.

### Presentation to User

The OTP is presented to the user, often in the form of a numeric code.
The user enters this code for authentication purposes.

### Validation on the Server

The server, which also knows the shared secret key, performs the same steps 
(timestamp calculation, HMAC-SHA1 calculation, dynamic truncation, and OTP 
generation) to calculate an expected OTP.

The server compares the expected OTP with the OTP entered by the user.
If they match, the authentication is successful.

### Clock Drift Tolerance

TOTP accounts for a limited amount of clock drift by allowing the server and 
client to have a small time window within which OTPs are considered valid.

### Window Size

A server can validate OTPs generated within a certain time window, typically a 
few time intervals before and after the current time.
The size of this window depends on the server's clock drift tolerance.

### Security Considerations

RFC 6238 provides guidance on security considerations, such as key protection 
and entropy for secret keys.

TOTP provides an additional layer of security by combining something the user 
knows (the shared secret key) with something the user has (the current time) to 
generate a one-time password. 
This OTP changes over time, making it a reliable method for 2FA and other 
authentication mechanisms.

## What I Learned?

### Encoding Padding 

Are characters added to an encoded string to have a consistent length.

For example the length of an output-encoded string must be multiple of four.
So if necessary the encoder will one or two padding characters (=) to the end 
in order to meet this requirement.

### Little and Big Endian

Little Endian and Big Endian are two different ways to represent multi-byte 
data types like integers in computer memory.

#### Little Endian

Stores the least significant byte (LSB) at the lowest memory address.
0x12345678 will be stored as 78 56 34 12.

#### Big Endian

Store the most significant byte (MSB) at the lowest memory address.
0x12345678 will be stored as 12 34 56 78.

#### When and Why?

The choice between Little Endian and Big Endian is important for interoperability 
and data exchange between different computer architectures and systems. 
It impacts how data is read and written when transferring information between systems.

- **Performance** - Little Endian is often considered more efficient for most common 
operations because it allows the CPU to directly access the LSB, which is usually 
more frequently used in arithmetic operations.

- **Network Protocols** - Network protocols often specify a specific endianness 
to ensure consistency when data is transmitted between systems. For example, many 
Internet protocols use Big Endian.

- **File Formats** - Some file formats may use a specific endianness, and data 
must be read or written in the correct endianness to interpret the file correctly.

## Resources

- [Writing a TOTP client in Go](https://rednafi.com/go/totp_client)
- [Rust Implement 2FA authentication](https://github.com/wpcodevo/totp-rust) 
- [Time Based One-time Password Algorithm - RFC6238](https://datatracker.ietf.org/doc/html/rfc6238) 

