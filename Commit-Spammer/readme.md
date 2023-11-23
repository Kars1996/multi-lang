# Mass Commit Script Documentation

## Description

This script automates the process of making multiple commits to a Git repository. If the Git repository is not initialized, it prompts the user to initialize it. It then creates a file, adds a series of edits to it, commits each edit, and pushes the changes to the 'master' branch. Initialy made in Python and may not work in other languages.

## Prerequisites

- Git installed on your machine.
- Python installed on your machine.

## Usage

1. Run the script in a directory where you want to create commits.
2. If a Git repository is not initialized, the script prompts the user to initialize it.
3. Follow the prompts to input the number of commits you'd like to make.

**Note:** Press `Control + C` at any time to stop the commit creation.

## Script Flow

1. Check if the Git repository is initialized. If not, prompt the user to initialize it.
2. Check if the specified file (`file.txt`) exists. If not, create and initialize it.
3. Clear the console screen.
4. Prompt the user for the number of commits.
5. Start a loop to create commits.
6. Add an edit to the file, stage it, and commit it for the specified number of times.
7. Push the changes to the 'master' branch.

## Exiting

- If the script is interrupted by the user (via `Control + C`), it will push the changes up to the point of interruption.

## Contribution Guide

If you would like to contribute to this script, please follow these guidelines:

1. Fork the repository.
2. Clone the forked repository to your local machine.
3. Create a new branch for your changes: `git checkout -b feature/new-feature`.
4. Make your changes and test them thoroughly.
5. Commit your changes: `git commit -m "Add your descriptive commit message"`.
6. Push your changes to your forked repository: `git push origin feature/new-feature`.
7. Create a pull request from your forked repository to the original repository.

Thank you for considering contributing to this project!