
# SHA256 Cracker

This Rust program is a simple password cracker designed to find a password corresponding to a given SHA-256 hash from a provided wordlist. It takes the following command-line arguments: the target hash, the path to the wordlist file, and executes the process. The program reads the wordlist, hashes each word using SHA-256, and compares the generated hash with the given target hash, printing the found password if a match is found or indicating that the password was not found.
