import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.io.InputStreamReader;

public class MassCommit {

    public static void main(String[] args) {
        String fileName = "file.txt";
        String commitMessage = "Mass Commit, No:";

        File gitDirectory = new File(".git");
        if (gitDirectory.isDirectory()) {
            System.out.println("Repository Initialized Already");
        } else {
            System.out.println("We would recommend initializing a GitHub repository.");
            System.out.println("Either use \"git initialise\" or use GitHub Desktop.");
            System.out.println("If you would like us to do it for you, press y, otherwise press n");

            String choice;
            do {
                System.out.print("> ");
                choice = System.console().readLine().toLowerCase();
            } while (!(choice.matches("[yn]") && choice.length() == 1));

            if (choice.equals("y")) {
                System.out.println("Initializing Repository.");
                executeCommand("git", "init");
                File readmeFile = new File("README.md");
                try {
                    readmeFile.createNewFile();
                } catch (IOException e) {
                    e.printStackTrace();
                }
                executeCommand("git", "add", "README.md");
                executeCommand("git", "commit", "-m", "Initial commit");
                System.out.println("We have initialized the repository. Simply import it to GitHub to use this program");
                return;
            }
        }

        File file = new File(fileName);
        if (file.exists()) {
            System.out.println("File Exists. Continuing");
        } else {
            try {
                file.createNewFile();
            } catch (IOException e) {
                e.printStackTrace();
            }
            System.out.println("File created: " + fileName);
        }

        System.out.println("Mass Commits, By Kars.");
        System.out.println("Please Input the amount of commits you would like. (Recommended 10-100 for speed)");

        String commitsInput;
        do {
            System.out.print("> ");
            commitsInput = System.console().readLine();
        } while (!commitsInput.matches("\\d+"));

        int commits = Integer.parseInt(commitsInput);

        System.out.println("Starting Mass Commits. Press Control + C at any time to stop the commit creation.");

        try {
            for (int i = 1; i < commits; i++) {
                executeCommand("git", "add", fileName);
                executeCommand("git", "commit", "-m", commitMessage + " " + i);
            }

            executeCommand("git", "push", "origin", "master");
        } catch (Exception e) {
            executeCommand("git", "push", "origin", "master");
            System.out.println("Ended at " + (commits - 1) + " commits");
        }

        System.out.println("Thank you for using my Commit thingy");
    }

    private static void executeCommand(String... command) {
        ProcessBuilder processBuilder = new ProcessBuilder(command);
        processBuilder.redirectErrorStream(true);

        try {
            Process process = processBuilder.start();
            BufferedReader reader = new BufferedReader(new InputStreamReader(process.getInputStream()));
            String line;
            while ((line = reader.readLine()) != null) {
                System.out.println(line);
            }

            process.waitFor();
        } catch (IOException | InterruptedException e) {
            e.printStackTrace();
        }
    }
}
