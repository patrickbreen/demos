This is my implementation of RSA encryption. I'm planning to implement a simple but functional version of RSA like this:

https://gist.github.com/JonCooperWorks/5314103

but I want to also add some block cipher modes, or maybe just variable block size for a single block.


Don't look at this project for good memory usage or performance. I'm using a library for big int that looks like I can't pass references into it, so I'm doing a lot of copying to make it work. Again, this is a hobby project.


Tasks:
- [x] Key Generation (from p and q)
- [x] Encryption/Decryption with 64 bit blocks
- [x] Improve performance so that 2048 bit RSA key generation is feasible if you already know p and q
        use ramp crate 
        use rust-gmp crate 
        maybe use apint crate?
        better memory allocation (less of it)
- [-] A method to generate p and q
- [-] Block modes (maybe))
