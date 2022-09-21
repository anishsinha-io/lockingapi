### Locking API Minimal Example

What's different about this API than the one in the previous repository?

- This implementation makes use of `parking_lot` RwLocks rather than implementations in `std`. The reasons for this are that they have more features. RwLocks in `parking_lot` allow for the creation of upgradable readers that can be atomically upgraded to be writers. It is also much *much* easier to use raw locks outside of RAII guards. While normally you don't want this because it's unsafe, this project necessitates locking a RwLock in one function and releasing it in another. This is critical and very difficult with the implementation in `std` which basically requires RAII guards to use. 
- This API is more fully featured and can be integrated into the rest of the code base easily without a billion `if let` statements. This API will be much easier to work with. 

What this API does:

- Provides functions for locking in 3 modes: `shared`, `shared_upgradable`, and `exclusive`, as well as functions to release/upgrade as necessary.
- Performs the locking raw. No RAII guards are present, which makes this API inherently unsafe, and up to the programmer to control undefined behavior as Rust's normal protections will not be there.

What the example does:

- Creates a thread pool with 4 readers, 2 sometimes writers, and 2 definite writers. The readers acquire read locks and print the value in the protected struct. The writers increment the value by one, and the sometimes writers acquire an upgradable lock, decide whether to write, and either upgrade their lock and write, or don't and release their upgradable lock. 
- The program runs in an infinite loop so just stop the program after 30 seconds and take a look at the output.


Why did this take so long to do?

- Because figuring out how to use `parking_lot` was really hard as the documentation does not provide many examples.

