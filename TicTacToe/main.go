package main

import (
	"fmt"
	"math/rand"
	"os"
	"os/exec"
	"strconv"
	"strings"
	"time"
)

func clearConsole() {
	cmd := exec.Command("clear")
	if os.Name == "windows" {
		cmd = exec.Command("cmd", "/c", "cls")
	}
	cmd.Stdout = os.Stdout
	cmd.Run()
}

func printBoard(board [][]string) {
	fmt.Println("  0 1 2")
	for i, row := range board {
		fmt.Printf("%d %s\n", i, strings.Join(row, " "))
	}
}

func isMoveValid(board [][]string, row, col int) bool {
	return board[row][col] == " "
}

func isBoardFull(board [][]string) bool {
	for _, row := range board {
		for _, cell := range row {
			if cell == " " {
				return false
			}
		}
	}
	return true
}

func checkWinner(board [][]string, player string) bool {
	for i := 0; i < 3; i++ {
		if board[i][0] == player && board[i][1] == player && board[i][2] == player {
			return true
		}
		if board[0][i] == player && board[1][i] == player && board[2][i] == player {
			return true
		}
	}
	if board[0][0] == player && board[1][1] == player && board[2][2] == player {
		return true
	}
	if board[0][2] == player && board[1][1] == player && board[2][0] == player {
		return true
	}
	return false
}

func getAvailableMoves(board [][]string) []string {
	var moves []string
	for i, row := range board {
		for j, cell := range row {
			if cell == " " {
				moves = append(moves, fmt.Sprintf("%d %d", i, j))
			}
		}
	}
	return moves
}

func makeComputerMove(board [][]string) (int, int) {
	availableMoves := getAvailableMoves(board)
	rand.Seed(time.Now().UnixNano())
	move := availableMoves[rand.Intn(len(availableMoves))]
	moveParts := strings.Split(move, " ")
	row, _ := strconv.Atoi(moveParts[0])
	col, _ := strconv.Atoi(moveParts[1])
	return row, col
}

func makePlayerMove(board [][]string) (int, int) {
	var row, col int
	for {
		fmt.Print("Enter your move (row and column, e.g., 0 1): ")
		_, err := fmt.Scanf("%d %d", &row, &col)
		if err == nil && 0 <= row && row < 3 && 0 <= col && col < 3 && isMoveValid(board, row, col) {
			break
		} else {
			fmt.Println("Invalid move. Try again.")
		}
	}
	return row, col
}

func playGame() {
	board := [][]string{{" ", " ", " "}, {" ", " ", " "}, {" ", " ", " "}}
	player := "X"
	computer := "O"
	clearConsole()

	for {
		printBoard(board)

		var row, col int
		if player == "X" {
			row, col = makePlayerMove(board)
		} else {
			row, col = makeComputerMove(board)
		}

		board[row][col] = player

		if checkWinner(board, player) {
			clearConsole()
			printBoard(board)
			fmt.Printf("%s wins!\n", player)
			break
		}

		if isBoardFull(board) {
			clearConsole()
			printBoard(board)
			fmt.Println("It's a tie!")
			break
		}

		player = "O"
		clearConsole()
	}
}

func main() {
	playGame()
}
