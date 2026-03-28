# Rust System Monitor
A lightweight rust system monitor, that allows you to search for a process by its name or list every process with a specific letter or keyword in it.

## how to use it
```bash
cargo run
```
This allow you to run the program in "normal mode" and seeing every current info about the system.
```bash
cargo run -- -s firefox.exe
```
Now we are trying to find the process "firefox.exe" and find its infos, if firefox isn't executed, the programs panics.

```bash
cargo run -- -lf fire
```
Now we are looking for every process with the eord "fire" in it, if it doesn't found anything it returnan error.