**DISCLAIMER:**

This code demonstrates password hashing verification but should **never** be used for malicious purposes like password cracking. It's ethically wrong and potentially illegal to attempt cracking passwords without authorization. Password hashing is a one-way process, meaning recovering the original password from the hash is computationally expensive but not impossible. This script highlights the importance of using strong password hashing algorithms and unique, complex passwords to enhance security.

## SHA-256 Hash Verification (Rust)

**Purpose:**

This Rust program demonstrates SHA-256 hash verification. It reads a password list and checks each password's hash against a provided SHA-256 hash (**for educational purposes only**).

**How it Works:**

1. Takes a single command-line argument: the SHA-256 hash to verify.
2. Reads a password list from the `src/passwordlist.txt` file.
3. Iterates over each password in the list:
   - Calculates the SHA-256 hash of the current password.
   - Compares the calculated hash to the provided hash.
4. If a match is found, the program exits and prints the password along with the number of attempts it took.
5. If no match is found after checking all passwords, the program exits and indicates failure.

**Important Note:**

* This is for educational purposes only. **Do not use this script for password cracking.**
* Password hashing is a one-way process. Recovering the original password from a hash is computationally difficult but not impossible.
* This script emphasizes the importance of strong password hashing algorithms and complex passwords for enhanced security.

**Usage:**

1. Clone this repository.
2. Open a terminal and navigate to the project directory.
3. Compile the project using `cargo build`.
4. Run the program with a valid SHA-256 hash as an argument: `cargo run <sha256_hash>` (Replace `<sha256_hash>` with the actual hash).

**Example:**

```
cargo run 38b060a75b5a0bd1e7d009ceda760a1c36c8fb455e4680526acc9c7c842c7e0b
```

**Disclaimer (again):**

Using this script for password cracking is unethical and potentially illegal. Consider alternative approaches for password security, such as password hashing with secure algorithms and encouraging users to create strong passwords.
