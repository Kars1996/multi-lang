using System;

class Program
{
    static void Main()
    {
        char[,] board = new char[3, 3] { { ' ', ' ', ' ' }, { ' ', ' ', ' ' }, { ' ', ' ', ' ' } };
        char player = 'X';
        char computer = 'O';
        
        ClearConsole();

        while (true)
        {
            PrintBoard(board);

            if (player == 'X')
            {
                int[] move = MakePlayerMove(board);
                int row = move[0];
                int col = move[1];
                board[row, col] = player;
            }
            else
            {
                int[] move = MakeComputerMove(board);
                int row = move[0];
                int col = move[1];
                board[row, col] = player;
            }

            if (CheckWinner(board, player))
            {
                ClearConsole();
                PrintBoard(board);
                Console.WriteLine($"{player} wins!");
                break;
            }

            if (IsBoardFull(board))
            {
                ClearConsole();
                PrintBoard(board);
                Console.WriteLine("It's a tie!");
                break;
            }

            player = (player == 'X') ? 'O' : 'X';
            ClearConsole();
        }
    }

    static void ClearConsole()
    {
        Console.Clear();
    }

    static void PrintBoard(char[,] board)
    {
        Console.WriteLine("  0 1 2");
        for (int i = 0; i < 3; i++)
        {
            Console.Write($"{i} ");
            for (int j = 0; j < 3; j++)
            {
                Console.Write($"{board[i, j]} ");
            }
            Console.WriteLine();
        }
    }

    static bool IsMoveValid(char[,] board, int row, int col)
    {
        return board[row, col] == ' ';
    }

    static bool IsBoardFull(char[,] board)
    {
        foreach (var cell in board)
        {
            if (cell == ' ')
            {
                return false;
            }
        }
        return true;
    }

    static bool CheckWinner(char[,] board, char player)
    {
        for (int i = 0; i < 3; i++)
        {
            if ((board[i, 0] == player && board[i, 1] == player && board[i, 2] == player) ||
                (board[0, i] == player && board[1, i] == player && board[2, i] == player))
            {
                return true;
            }
        }

        if ((board[0, 0] == player && board[1, 1] == player && board[2, 2] == player) ||
            (board[0, 2] == player && board[1, 1] == player && board[2, 0] == player))
        {
            return true;
        }

        return false;
    }

    static int[] MakePlayerMove(char[,] board)
    {
        while (true)
        {
            try
            {
                Console.Write("Enter your move (row and column, e.g., 0 1): ");
                string[] input = Console.ReadLine().Split();
                int row = int.Parse(input[0]);
                int col = int.Parse(input[1]);

                if (0 <= row && row < 3 && 0 <= col && col < 3 && IsMoveValid(board, row, col))
                {
                    return new int[] { row, col };
                }
                else
                {
                    Console.WriteLine("Invalid move. Try again.");
                }
            }
            catch (FormatException)
            {
                Console.WriteLine("Invalid input. Please enter two integers separated by a space.");
            }
        }
    }

    static int[] MakeComputerMove(char[,] board)
    {
        Random random = new Random();
        int row, col;

        do
        {
            row = random.Next(0, 3);
            col = random.Next(0, 3);
        } while (!IsMoveValid(board, row, col));

        return new int[] { row, col };
    }
}
