#include <iostream>
#include <vector>
#include <cstdlib>
#include <ctime>

void clearConsole() {
#ifdef _WIN32
    system("cls");
#else
    system("clear");
#endif
}

void printBoard(const std::vector<std::vector<char>>& board) {
    std::cout << "  0 1 2\n";
    for (int i = 0; i < 3; ++i) {
        std::cout << i << " ";
        for (int j = 0; j < 3; ++j) {
            std::cout << board[i][j] << " ";
        }
        std::cout << "\n";
    }
}

bool isMoveValid(const std::vector<std::vector<char>>& board, int row, int col) {
    return board[row][col] == ' ';
}

bool isBoardFull(const std::vector<std::vector<char>>& board) {
    for (const auto& row : board) {
        for (char cell : row) {
            if (cell == ' ') {
                return false;
            }
        }
    }
    return true;
}

bool checkWinner(const std::vector<std::vector<char>>& board, char player) {
    for (int i = 0; i < 3; ++i) {
        if (board[i][0] == player && board[i][1] == player && board[i][2] == player) {
            return true;
        }
        if (board[0][i] == player && board[1][i] == player && board[2][i] == player) {
            return true;
        }
    }
    if (board[0][0] == player && board[1][1] == player && board[2][2] == player) {
        return true;
    }
    if (board[0][2] == player && board[1][1] == player && board[2][0] == player) {
        return true;
    }
    return false;
}

std::vector<std::pair<int, int>> getAvailableMoves(const std::vector<std::vector<char>>& board) {
    std::vector<std::pair<int, int>> availableMoves;
    for (int i = 0; i < 3; ++i) {
        for (int j = 0; j < 3; ++j) {
            if (board[i][j] == ' ') {
                availableMoves.push_back(std::make_pair(i, j));
            }
        }
    }
    return availableMoves;
}

std::pair<int, int> makeComputerMove(const std::vector<std::vector<char>>& board) {
    auto availableMoves = getAvailableMoves(board);
    srand(static_cast<unsigned>(time(nullptr)));
    return availableMoves[rand() % availableMoves.size()];
}

std::pair<int, int> makePlayerMove(const std::vector<std::vector<char>>& board) {
    while (true) {
        try {
            int row, col;
            std::cout << "Enter your move (row and column, e.g., 0 1): ";
            std::cin >> row >> col;

            if (row >= 0 && row < 3 && col >= 0 && col < 3 && isMoveValid(board, row, col)) {
                return std::make_pair(row, col);
            } else {
                std::cout << "Invalid move. Try again.\n";
            }
        } catch (...) {
            std::cout << "Invalid input. Please enter two integers separated by a space.\n";
            std::cin.clear();
            std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
        }
    }
}

void playGame() {
    std::vector<std::vector<char>> board(3, std::vector<char>(3, ' '));
    char player = 'X';
    char computer = 'O';
    clearConsole();

    while (true) {
        printBoard(board);

        std::pair<int, int> move;
        if (player == 'X') {
            move = makePlayerMove(board);
        } else {
            move = makeComputerMove(board);
        }

        board[move.first][move.second] = player;

        if (checkWinner(board, player)) {
            clearConsole();
            printBoard(board);
            std::cout << player << " wins!\n";
            break;
        }

        if (isBoardFull(board)) {
            clearConsole();
            printBoard(board);
            std::cout << "It's a tie!\n";
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
