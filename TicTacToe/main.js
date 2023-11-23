const readline = require("readline");

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
});

const board = [
    [" ", " ", " "],
    [" ", " ", " "],
    [" ", " ", " "],
];

const player = "X";
const computer = "O";

function clearConsole() {
    console.clear();
}

function printBoard() {
    console.log("  0 1 2");
    for (let i = 0; i < 3; i++) {
        console.log(`${i} ${board[i].join(" ")}`);
    }
}

function isMoveValid(row, col) {
    return board[row][col] === " ";
}

function isBoardFull() {
    return board.every((row) => row.every((cell) => cell !== " "));
}

function checkWinner(player) {
    for (let i = 0; i < 3; i++) {
        if (
            board[i][0] === player &&
            board[i][1] === player &&
            board[i][2] === player
        ) {
            return true;
        }
    }

    for (let i = 0; i < 3; i++) {
        if (
            board[0][i] === player &&
            board[1][i] === player &&
            board[2][i] === player
        ) {
            return true;
        }
    }

    if (
        board[0][0] === player &&
        board[1][1] === player &&
        board[2][2] === player
    ) {
        return true;
    }
    if (
        board[0][2] === player &&
        board[1][1] === player &&
        board[2][0] === player
    ) {
        return true;
    }

    return false;
}

function getAvailableMoves() {
    const moves = [];
    for (let i = 0; i < 3; i++) {
        for (let j = 0; j < 3; j++) {
            if (board[i][j] === " ") {
                moves.push({ row: i, col: j });
            }
        }
    }
    return moves;
}

function makeComputerMove() {
    const availableMoves = getAvailableMoves();
    const randomMove =
        availableMoves[Math.floor(Math.random() * availableMoves.length)];
    return randomMove;
}

function makePlayerMove() {
    return new Promise((resolve) => {
        rl.question(
            "Enter your move (row and column, e.g., 0 1): ",
            (input) => {
                const [row, col] = input
                    .split(" ")
                    .map((coord) => parseInt(coord, 10));
                if (
                    isNaN(row) ||
                    isNaN(col) ||
                    row < 0 ||
                    row >= 3 ||
                    col < 0 ||
                    col >= 3 ||
                    !isMoveValid(row, col)
                ) {
                    console.log("Invalid move. Try again.");
                    makePlayerMove().then(resolve);
                } else {
                    resolve({ row, col });
                }
            }
        );
    });
}

async function playGame() {
    clearConsole();
    let currentPlayer = player;

    while (true) {
        printBoard();

        let move;
        if (currentPlayer === player) {
            move = await makePlayerMove();
        } else {
            move = makeComputerMove();
        }

        board[move.row][move.col] = currentPlayer;

        if (checkWinner(currentPlayer)) {
            clearConsole();
            printBoard();
            console.log(`${currentPlayer} wins!`);
            break;
        }

        if (isBoardFull()) {
            clearConsole();
            printBoard();
            console.log("It's a tie!");
            break;
        }

        currentPlayer = currentPlayer === player ? computer : player;
        clearConsole();
    }

    rl.close();
}

playGame();
