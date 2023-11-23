#include <iostream>
#include <fstream>
#include <cstdlib>
#include <cstdio>
#include <sstream>

const std::string file_name = "file.txt";
const std::string commit_message = "Mass Commit, No:";

void initializeRepository() {
    std::cout << "Repository Initialized Already" << std::endl;
}

void initializeFile() {
    std::ofstream file(file_name);
    file << "File Initialized";
    file.close();
    std::cout << "File created: " << file_name << std::endl;
}

int main() {
    if (std::filesystem::exists(".git")) {
        initializeRepository();
    } else {
        std::cout << "We would recommend initializing a GitHub repository." << std::endl;
        std::cout << "Either use \"git initialise\" or use GitHub Desktop." << std::endl;
        std::cout << "If you would like us to do it for you press y, otherwise press n" << std::endl;
        
        char choice;
        do {
            std::cout << "> ";
            std::cin >> choice;
        } while (!std::isalpha(choice) || !std::isascii(choice) || std::cin.fail());

        if (choice == 'y') {
            std::cout << "Initializing Repository." << std::endl;
            system("git init");
            std::ofstream file("README.md");
            file << "# Commit";
            file.close();
            system("git add README.md");
            system("git commit -m \"Initial commit\"");
            std::cout << "Repository initialized. Simply import it to GitHub to use this program." << std::endl;
            return 0;
        }
    }

    if (std::filesystem::exists(file_name)) {
        std::cout << "File Exists. Continuing" << std::endl;
    } else {
        initializeFile();
    }

    system("cls");

    std::cout << "Mass Commits, By Kars." << std::endl;
    std::cout << "Please Input the amount of commits you would like. (Recommended 10-100 for speed)" << std::endl;

    int commits;
    do {
        std::cout << "> ";
        std::cin >> commits;
    } while (std::cin.fail());

    std::cout << "Starting Mass Commits. Press Control + C at any time to stop the commit creation." << std::endl;

    try {
        for (int i = 1; i < commits; ++i) {
            std::ofstream file(file_name, std::ios::app);
            file << "Edit " << i << "\n";
            file.close();
            system(("git add " + file_name).c_str());
            system(("git commit -m \"" + commit_message + " " + std::to_string(i) + "\"").c_str());
        }
        system("git push origin master");
    } catch (...) {
        system("git push origin master");
        std::cout << "Ended at " << commits - 1 << " commits" << std::endl;
    }

    std::cout << "Thank you for using my Commit thingy" << std::endl;

    return 0;
}
