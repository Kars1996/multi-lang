#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define FILE_NAME "file.txt"
#define COMMIT_MESSAGE "Mass Commit, No:"

void initializeRepository() {
    printf("Repository Initialized Already\n");
}

void initializeFile() {
    FILE *file = fopen(FILE_NAME, "w");
    fprintf(file, "File Initialized");
    fclose(file);
    printf("File created: %s\n", FILE_NAME);
}

int main() {
    if (access(".git", F_OK) == 0) {
        initializeRepository();
    } else {
        printf("We would recommend initializing a GitHub repository.\n");
        printf("Either use \"git initialise\" or use GitHub Desktop.\n");
        printf("If you would like us to do it for you press y, otherwise press n\n");

        char choice;
        do {
            printf("> ");
            scanf(" %c", &choice);
        } while (!((choice >= 'a' && choice <= 'z') || (choice >= 'A' && choice <= 'Z')));

        if (choice == 'y') {
            printf("Initializing Repository.\n");
            system("git init");
            
            FILE *file = fopen("README.md", "w");
            fprintf(file, "# Commit");
            fclose(file);

            system("git add README.md");
            system("git commit -m \"Initial commit\"");
            
            printf("Repository initialized. Simply import it to GitHub to use this program.\n");
            return 0;
        }
    }

    if (access(FILE_NAME, F_OK) == 0) {
        printf("File Exists. Continuing\n");
    } else {
        initializeFile();
    }

    system("clear");

    printf("Mass Commits, By Kars.\n");
    printf("Please Input the amount of commits you would like. (Recommended 10-100 for speed)\n");

    int commits;
    do {
        printf("> ");
        scanf("%d", &commits);
    } while (commits <= 0);

    printf("Starting Mass Commits. Press Control + C at any time to stop the commit creation.\n");

    int i;
    for (i = 1; i < commits; ++i) {
        FILE *file = fopen(FILE_NAME, "a");
        fprintf(file, "Edit %d\n", i);
        fclose(file);

        system("git add file.txt");
        char commit_command[100];
        sprintf(commit_command, "git commit -m \"%s %d\"", COMMIT_MESSAGE, i);
        system(commit_command);
    }

    system("git push origin master");

    printf("Thank you for using my Commit thingy\n");

    return 0;
}
