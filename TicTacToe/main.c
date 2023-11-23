#include <stdio.h>
#include <stdlib.h>
#include <time.h>

void clearConsole() {
    system("clear || cls");
}

void printBoard(char board[3][3]) {
    printf("  0 1 2\n");
    for (int i = 0; i < 3; i++) {
        printf("%d ", i);
        for (int j = 0; j < 3; j++) {
            printf("%c ", board[i][j]);
        }
        printf("\n");
    }
}

int isMoveValid(char board[3][3], int row, int col) {
    return board[row][col] == ' ';
}

int isBoardFull(char board[3][3]) {
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            if (board[i][j] == ' ') {
                return 0; // Not full
            }
        }
    }
    return 1; // Full
}

int checkWinner(char board[3][3], char player) {
    for (int i = 0; i < 3; i++) {
        if ((board[i][0] == player && board[i][1] == player && board[i][2] == player) ||
            (board[0][i] == player && board[1][i] == player && board[2][i] == player)) {
            return 1; // Winner
        }
    }
    if ((board[0][0] == player && board[1][1] == player && board[2][2] == player) ||
        (board[0][2] == player && board[1][1] == player && board[2][0] == player)) {
        return 1; // Winner
    }
    return 0; // No winner
}

void getAvailableMoves(char board[3][3], int availableMoves[9][2], int* numMoves) {
    *numMoves = 0;
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            if (board[i][j] == ' ') {
                availableMoves[*numMoves][0] = i;
                availableMoves[*numMoves][1] = j;
                (*numMoves)++;
            }
        }
    }
}

void makeComputerMove(char board[3][3]) {
    int availableMoves[9][2];
    int numMoves;
    getAvailableMoves(board, availableMoves, &numMoves);

    srand(time(NULL));
    int moveIndex = rand() % numMoves;

    int row = availableMoves[moveIndex][0];
    int col = availableMoves[moveIndex][1];

    board[row][col] = 'O';
}

void makePlayerMove(char board[3][3]) {
    while (1) {
        int row, col;
        printf("Enter your move (row and column, e.g., 0 1): ");
        scanf("%d %d", &row, &col);

        if (row >= 0 && row < 3 && col >= 0 && col < 3 && isMoveValid(board, row, col)) {
            board[row][col] = 'X';
            break;
        } else {
            printf("Invalid move. Try again.\n");
        }
    }
}

void playGame() {
    char board[3][3] = {{' ', ' ', ' '}, {' ', ' ', ' '}, {' ', ' ', ' '}};
    char player = 'X';
    char computer = 'O';
    clearConsole();

    while (1) {
        printBoard(board);

        if (player == 'X') {
            makePlayerMove(board);
        } else {
            makeComputerMove(board);
        }

        if (checkWinner(board, player)) {
            clearConsole();
            printBoard(board);
            printf("%c wins!\n", player);
            break;
        }

        if (isBoardFull(board)) {
            clearConsole();
            printBoard(board);
            printf("It's a tie!\n");
            break;
        }

        player = (player == 'X') ? 'O' : 'X';
        clearConsole();
    }
}

int main() {
    playGame();
    return 0;
}
