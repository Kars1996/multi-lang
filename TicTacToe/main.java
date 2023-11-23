import java.util.Arrays;
import java.util.Random;
import java.util.Scanner;

public class TicTacToe {
    public static void clearConsole() {
        System.out.print("\033[H\033[2J");
        System.out.flush();
    }

    public static void printBoard(char[][] board) {
        System.out.println("  0 1 2");
        for (int i = 0; i < 3; i++) {
            System.out.println(i + " " + String.join(" ", Arrays.copyOf(board[i], 3)));
        }
    }

    public static boolean isMoveValid(char[][] board, int row, int col) {
        return board[row][col] == ' ';
    }

    public static boolean isBoardFull(char[][] board) {
        for (char[] row : board) {
            for (char cell : row) {
                if (cell == ' ') {
                    return false;
                }
            }
        }
        return true;
    }

    public static boolean checkWinner(char[][] board, char player) {
        for (int i = 0; i < 3; i++) {
            if (Arrays.equals(board[i], new char[]{player, player, player}) ||
                (board[0][i] == player && board[1][i] == player && board[2][i] == player)) {
                return true;
            }
        }
        return (board[0][0] == player && board[1][1] == player && board[2][2] == player) ||
               (board[0][2] == player && board[1][1] == player && board[2][0] == player);
    }

    public static int[] makeComputerMove(char[][] board) {
        int[] move = getAvailableMoves(board).get(new Random().nextInt(getAvailableMoves(board).size()));
        return new int[]{move[0], move[1]};
    }

    public static int[] makePlayerMove(char[][] board) {
        Scanner scanner = new Scanner(System.in);
        while (true) {
            try {
                System.out.print("Enter your move (row and column, e.g., 0 1): ");
                int row = scanner.nextInt();
                int col = scanner.nextInt();
                if (0 <= row && row < 3 && 0 <= col && col < 3 && isMoveValid(board, row, col)) {
                    return new int[]{row, col};
                } else {
                    System.out.println("Invalid move. Try again.");
                }
            } catch (Exception e) {
                System.out.println("Invalid input. Please enter two integers separated by a space.");
                scanner.nextLine(); // Consume invalid input
            }
        }
    }

    public static java.util.List<int[]> getAvailableMoves(char[][] board) {
        java.util.List<int[]> availableMoves = new java.util.ArrayList<>();
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                if (board[i][j] == ' ') {
                    availableMoves.add(new int[]{i, j});
                }
            }
        }
        return availableMoves;
    }

    public static void playGame() {
        char[][] board = new char[3][3];
        for (int i = 0; i < 3; i++) {
            Arrays.fill(board[i], ' ');
        }
        char player = 'X';
        char computer = 'O';
        clearConsole();

        while (true) {
            printBoard(board);

            int[] move;
            if (player == 'X') {
                move = makePlayerMove(board);
            } else {
                move = makeComputerMove(board);
            }

            board[move[0]][move[1]] = player;

            if (checkWinner(board, player)) {
                clearConsole();
                printBoard(board);
                System.out.println(player + " wins!");
                break;
            }

            if (isBoardFull(board)) {
                clearConsole();
                printBoard(board);
                System.out.println("It's a tie!");
                break;
            }

            player = (player == 'X') ? 'O' : 'X';
            clearConsole();
        }
    }

    public static void main(String[] args) {
        playGame();
    }
}
