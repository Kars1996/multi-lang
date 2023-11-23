# Tic-Tac-Toe Game Documentation

## Overview

This is a simple command-line Tic-Tac-Toe game initially written in Node.js. The game supports a human player (X) playing against a computer player (O). The players take turns making moves on a 3x3 game board, and the game continues until one player wins or the board is full (resulting in a tie). 

## File Structure

- `tic_tac_toe.js`: The main JavaScript file containing the game logic.
- `README.md`: This documentation file.

## How to Run

1. Open a terminal.
2. Navigate to the directory containing `tic_tac_toe.js`.
3. Run the following command:

    ```bash
    node tic_tac_toe.js
    ```

4. Follow the prompts to make your moves.

## Gameplay

- The game board is a 3x3 grid represented by coordinates (0,0) to (2,2).
- Players take turns making moves by entering row and column coordinates when prompted.
- The game continues until one player wins or the board is full.
- The player with the symbol "X" wins by having three of their symbols in a row (horizontally, vertically, or diagonally).
- The computer player with the symbol "O" employs a basic random move strategy.

## Functions and Features

### `clearConsole()`

- Clears the console to keep the display clean during gameplay.

### `printBoard()`

- Prints the current state of the game board to the console.

### `isMoveValid(row, col)`

- Checks if a move at the specified row and column is valid (the cell is empty).

### `isBoardFull()`

- Checks if the game board is full.

### `checkWinner(player)`

- Checks if the specified player has won the game.

### `getAvailableMoves()`

- Returns a list of available moves on the current board.

### `makeComputerMove()`

- Makes a random move for the computer player.

### `makePlayerMove()`

- Asks the human player for input and returns the chosen move.

### `playGame()`

- The main function that controls the flow of the game.

## Notes

- The game uses the `readline` module to handle user input.
- The console is cleared at the start of each turn to provide a clean interface.

## Contribution Guide

If you would like to contribute to this script, please follow these guidelines:

1. Fork the repository.
2. Clone the forked repository to your local machine.
3. Create a new branch for your changes: `git checkout -b feature/new-feature`.
4. Make your changes and test them thoroughly.
5. Commit your changes: `git commit -m "Add your descriptive commit message"`.
6. Push your changes to your forked repository: `git push origin feature/new-feature`.
7. Create a pull request from your forked repository to the original repository.

Enjoy playing Tic-Tac-Toe!
