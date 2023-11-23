import os
import random

def clear_console():
    os.system('clear' if os.name == 'posix' else 'cls')

def print_board(board):
    print("  0 1 2")
    for i, row in enumerate(board):
        print(f"{i} {' '.join(row)}")

def is_move_valid(board, row, col):
    return board[row][col] == " "

def is_board_full(board):
    return all(cell != " " for row in board for cell in row)

def check_winner(board, player):
    for i in range(3):
        if all(board[i][j] == player for j in range(3)) or all(board[j][i] == player for j in range(3)):
            return True
    if all(board[i][i] == player for i in range(3)) or all(board[i][2 - i] == player for i in range(3)):
        return True
    return False

def get_available_moves(board):
    return [(i, j) for i in range(3) for j in range(3) if board[i][j] == " "]

def make_computer_move(board):
    available_moves = get_available_moves(board)
    return random.choice(available_moves)

def make_player_move(board):
    while True:
        try:
            row, col = map(int, input("Enter your move (row and column, e.g., 0 1): ").split())
            if 0 <= row < 3 and 0 <= col < 3 and is_move_valid(board, row, col):
                return row, col
            else:
                print("Invalid move. Try again.")
        except ValueError:
            print("Invalid input. Please enter two integers separated by a space.")

def play_game():
    board = [[" " for _ in range(3)] for _ in range(3)]
    player = "X"
    computer = "O"
    clear_console()

    while True:
        print_board(board)

        if player == "X":
            row, col = make_player_move(board)
        else:
            row, col = make_computer_move(board)

        board[row][col] = player

        if check_winner(board, player):
            clear_console()
            print_board(board)
            print(f"{player} wins!")
            break

        if is_board_full(board):
            clear_console()
            print_board(board)
            print("It's a tie!")
            break

        player = "O" if player == "X" else "X"
        clear_console()

if __name__ == "__main__":
    play_game()
