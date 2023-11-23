import * as readlineSync from 'readline-sync';

function clearConsole() {
    console.clear();
}

function printBoard(board: string[][]) {
    console.log("  0 1 2");
    for (let i = 0; i < 3; i++) {
        console.log(`${i} ${board[i].join(' ')}`);
    }
}

function isMoveValid(board: string[][], row: number, col: number): boolean {
    return board[row][col] === " ";
}

function isBoardFull(board: string[][]): boolean {
    return board.every(row => row.every(cell => cell !== " "));
}

function checkWinner(board: string[][], player: string): boolean {
    for (let i = 0; i < 3; i++) {
        if (board[i].every(cell => cell === player) || board.every(row => row[i] === player)) {
            return true;
        }
    }

    if (board.every((row, i) => row[i] === player) || board.every((row, i) => row[2 - i] === player)) {
        return true;
    }

    return false;
}

function getAvailableMoves(board: string[][]): [number, number][] {
    const moves: [number, number][] = [];
    for (let i = 0; i < 3; i++) {
        for (let j = 0; j < 3; j++) {
            if (board[i][j] === " ") {
                moves.push([i, j]);
            }
        }
    }
    return moves;
}

function makeComputerMove(board: string[][]): [number, number] {
    const availableMoves = getAvailableMoves(board);
    const randomIndex = Math.floor(Math.random() * availableMoves.length);
    return availableMoves[randomIndex];
}

function makePlayerMove(board: string[][]): [number, number] {
    while (true) {
        try {
            const input = readlineSync.question("Enter your move (row and column, e.g., 0 1): ");
            const [row, col] = input.split(' ').map(Number);

            if (0 <= row && row < 3 && 0 <= col && col < 3 && isMoveValid(board, row, col)) {
                return [row, col];
            } else {
                console.log("Invalid move. Try again.");
            }
        } catch (error) {
            console.log("Invalid input. Please enter two integers separated by a space.");
        }
    }
}

function playGame() {
    const board: string[][] = [[" ", " ", " "], [" ", " ", " "], [" ", " ", " "]];
    let player = "X";
    const computer = "O";
    clearConsole();

    while (true) {
        printBoard(board);

        const [row, col] = (player === "X") ? makePlayerMove(board) : makeComputerMove(board);
        board[row][col] = player;

        if (checkWinner(board, player)) {
            clearConsole();
            printBoard(board);
            console.log(`${player} wins!`);
            break;
        }

        if (isBoardFull(board)) {
            clearConsole();
            printBoard(board);
            console.log("It's a tie!");
            break;
        }

        player = (player === "X") ? "O" : "X";
        clearConsole();
    }
}

playGame();
