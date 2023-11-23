@echo off

set file_name=file.txt
set commit_message=Mass Commit, No:

if exist ".git" (
    echo Repository Initialised Already
) else (
    echo We would recommend initialising a GitHub repository.
    echo Either use "git initialise" or use GitHub Desktop.
    echo If you would like us to do it for you press y, otherwise press n
    set /p choice=^> 
    :validate_choice
    if /i not "%choice%"=="y" if /i not "%choice%"=="n" (
        echo Please input a single letter. Either Y or N
        set /p choice=^> 
        goto validate_choice
    )
    if /i "%choice%"=="y" (
        echo Initialising Repository.
        git init
        echo # Commit > README.md
        git add README.md
        git commit -m "Initial commit"
        echo We have initialised the repository. Simply import it to GitHub to use this program.
        exit /b
    )
)

if exist "%file_name%" (
    echo File Exists. Continuing
) else (
    echo File Initialised > %file_name%
    echo File created: %file_name%
)

cls
echo Mass Commits, By Kars.
echo Please Input the amount of commits you would like. (Recommended 10-100 for speed)

:validate_commits
set /p commits=^> 
if not "%commits%"=="%commits:~0,1%" (
    echo Please Input a Number
    goto validate_commits
)

echo Starting Mass Commits. Press Control + C at any time to stop the commit creation.

setlocal enabledelayedexpansion

for /l %%i in (1,1,%commits%) do (
    echo Edit %%i>> %file_name%
    git add %file_name%
    git commit -m "%commit_message% %%i"
)

git push origin master

endlocal

echo Thank you for using my Commit thingy
