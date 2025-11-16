# Overview

In this program, I wanted to try and replicate one of my favorite video games, Tetris. Tetris was created by Russian programmer Alexey Pajitnov in the mid-1980's, but has seen many releases and versions throughout the years and remains widely popular today. I have always loved Tetris for its simplicity and ease of play, yet it is also very addicting and challenging despite its simple design and gameplay. I wanted to use this game as a template by which to learn a new programming language, Rust, while recreating the game.

The software that I have written is a basic clone of the popular video game, Tetris. In this game, you play within a vertical grid. During gameplay, a series of geometric blocks, called tetraminos, descend through the grid until they reach the bottom. While a tetramino is in motion you are able to mive it, rotating it left/right, and shifting it left/right, determining which position the tetramino will be in when it reaches the bottom of the grid. Your goal is to strategically fit the tetraminos together so that they make complete horizontal lines across the width of the grid. Once a line is made, it will disappear, giving you more room to place the next falling tetraminos. The more lines you make, the higher your score will be. The game ends when you have stacked too many tetraminos on top of each other and they touch the top of the grid, leaving no room for new tetraminos. I wrote the programming for this game using the Rust language, which I have been studying and learning for the past two weeks. Because this is my own unofficial version of Teris, using the Rust language, I have created my own new version of the game: "TetRust"!

My purpose in writing this game was primarily to learn the Rust programming language, which I have been interested in learning for a long time. I ahve always been interested in low-level programming, but have always been cautious about diving into such languages as C and Assembly, which can be unforgiving and have a steep learning curve. Rust accomplishes a lot of the same tasks that can be done with C, but has the advantage of many features and tools that are generally only found in higher level languages, such as checks at compilation, and other such bug checkers. I thought it would be fun to practice my Rust knowledge by making a game, and since I am already very familiar with Tetris, and since its design and gameplay are already so simple, I thought it would be the perfect project for this purpose.

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

- VSCode for text editing and writing the programming
- Windows command line for installing Rust and dependencies, and running cargo commands

Language and libraries:
- Rust, a multi-purpose low-level language that utilizes certain tools and features of high-level languages
- macroquad, a simple game library for Rust
- rand, a Rust library which allows for random generation
- anyhow, a library that simplifies error handling in Rust programming
- serde, a library for serializing and deserializing Rust data structures

# Useful Websites

- [w3schools.com](https://www.w3schools.com/rust/index.php)
- [rust-lang.org](https://rust-lang.org/)
- [youtube.com](https://www.youtube.com/)

# Future Work

- Implement a 2 player mode
- Create a score saving feature that would allow players to see their past score
- Utilize music and sound effects in the game