# Number Memory Game 🧠

A simple Rust CLI game where the user must remember a randomly generated 4-digit number shown briefly on the screen.

## How it works

- A number between 1000–9999 is shown for 2 seconds.
- The screen is then cleared.
- The user is asked to recall and enter the number.
- If correct: 🎉 Win message. Otherwise: ❌ Try again!

## Tech

- Language: Rust
- Random: `rand::Rng`
- Input: `std::io`
- Timing: `std::thread::sleep`
