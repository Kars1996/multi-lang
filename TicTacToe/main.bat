@echo off
setlocal EnableDelayedExpansion

:clear_console
cls
goto :eof

:print_board
echo    0 1 2
for /L %%i in (0,1,2) do (
    set "row=!board[%%i]!"
    echo %%i !row: =!
)
goto :eof

:is_move_valid
set "cell=!board[%1][%2]!"
if "!cell!" == " " (
    exit /b 0
) else (
    exit /b 1
)

:is_board_full
for /L %%i in (0,1,2) do (
    for /L %%j in (0,1,2) do (
        set "cell=!board[%%i][%%j]!"
        if "!cell!" == " " (
            exit /b 0
        )
    )
)
exit /b 1

:check_winner
for /L %%i in (0,1,2) do (
    set "row=!board[%%i]!"
    set "col=!board[0][%%i]!!board[1][%%i]!!board[2][%%i]!"
    if "!row!" == "%1%1%1" exit /b 1
    if "!col!" == "%1%1%1" exit /b 1
)
set "diag1=!board[0][0]!!board[1][1]!!board[2][2]!"
set "diag2=!board[0][2]!!board[1][1]!!board[2][0]!"
if "!diag1!" == "%1%1%1" exit /b 1
if "!diag2!" == "%1%1%1" exit /b 1
exit /b 0

:get_available_moves
set "moves="
for /L %%i in (0,1,2) do (
    for /L %%j in (0,1,2) do (
        set "cell=!board[%%i][%%j]!"
        if "!cell!" == " " (
            set "moves=!moves! %%i %%j"
        )
    )
)
echo %moves%
goto :eof

:make_computer_move
call :get_available_moves
set /a moveIndex=%random%%%moves
for /f "tokens=1,2" %%i in ("!moves:~%moveIndex%,2!") do (
    set /a row=%%i, col=%%j
)
exit /b

:make_player_move
set /p "input=Enter your move (row and column, e.g., 0 1): "
for /f "tokens=1,2" %%i in ("%input%") do (
    set /a row=%%i, col=%%j
)
if %row% geq 0 if %row% lss 3 if %col% geq 0 if %col% lss 3 (
    call :is_move_valid %row% %col%
    if errorlevel 1 (
        echo Invalid move. Try again.
        goto :make_player_move
    )
) else (
    echo Invalid move. Try again.
    goto :make_player_move
)
goto :eof

:play_game
set "player=X"
set "computer=O"

set "board[0]=   "
set "board[1]=   "
set "board[2]=   "

call :clear_console

:game_loop
call :print_board
if "!player!" == "X" (
    call :make_player_move
) else (
    call :make_computer_move
)
set "board[%row%][%col%]=!player!"

call :check_winner !player!
if errorlevel 1 (
    call :clear_console
    call :print_board
    echo !player! wins!
    goto :eof
)

call :is_board_full
if errorlevel 1 (
    call :clear_console
    call :print_board
    echo It's a tie!
    goto :eof
)

set "player=O"
call :clear_console
goto :game_loop

:eof
