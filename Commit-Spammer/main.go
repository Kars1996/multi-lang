package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"
	"strings"
)

const (
	fileName       = "file.txt"
	commitMessage  = "Mass Commit, No:"
	repoInitialize = "y"
)

func initializeRepository() {
	fmt.Println("Repository Initialized Already")
}

func initializeFile() {
	err := ioutil.WriteFile(fileName, []byte("File Initialized"), 0644)
	if err != nil {
		fmt.Printf("Error creating file: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("File created: %s\n", fileName)
}

func main() {
	if _, err := os.Stat(".git"); err == nil {
		initializeRepository()
	} else {
		fmt.Println("We would recommend initializing a GitHub repository.")
		fmt.Println("Either use \"git initialise\" or use GitHub Desktop.")
		fmt.Println("If you would like us to do it for you press y, otherwise press n")

		var choice string
		for {
			fmt.Print("> ")
			fmt.Scan(&choice)
			if len(choice) == 1 && (choice[0] == 'y' || choice[0] == 'n') {
				break
			}
			fmt.Println("Please input a single letter. Either Y or N")
		}

		if strings.ToLower(choice) == repoInitialize {
			fmt.Println("Initializing Repository.")
			cmd := exec.Command("git", "init")
			cmd.Run()

			err := ioutil.WriteFile("README.md", []byte("# Commit"), 0644)
			if err != nil {
				fmt.Printf("Error creating README.md: %v\n", err)
				os.Exit(1)
			}

			cmd = exec.Command("git", "add", "README.md")
			cmd.Run()

			cmd = exec.Command("git", "commit", "-m", "Initial commit")
			cmd.Run()

			fmt.Println("Repository initialized. Simply import it to GitHub to use this program.")
			return
		}
	}

	if _, err := os.Stat(fileName); err == nil {
		fmt.Println("File Exists. Continuing")
	} else {
		initializeFile()
	}

	fmt.Println("Mass Commits, By Kars.")
	fmt.Println("Please Input the amount of commits you would like. (Recommended 10-100 for speed)")

	var commits int
	for {
		fmt.Print("> ")
		_, err := fmt.Scan(&commits)
		if err == nil && commits > 0 {
			break
		}
		fmt.Println("Please Input a Number")
	}

	fmt.Println("Starting Mass Commits. Press Control + C at any time to stop the commit creation.")

	for i := 1; i < commits; i++ {
		file, err := os.OpenFile(fileName, os.O_APPEND|os.O_WRONLY, 0644)
		if err != nil {
			fmt.Printf("Error opening file: %v\n", err)
			os.Exit(1)
		}
		defer file.Close()

		fmt.Fprintf(file, "Edit %d\n", i)

		cmd := exec.Command("git", "add", fileName)
		cmd.Run()

		cmd = exec.Command("git", "commit", "-m", fmt.Sprintf("%s %d", commitMessage, i))
		cmd.Run()
	}

	cmd := exec.Command("git", "push", "origin", "master")
	cmd.Run()

	fmt.Printf("Thank you for using my Commit thingy\n")
}
