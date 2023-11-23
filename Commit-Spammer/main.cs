using System;
using System.Diagnostics;
using System.IO;

class Program
{
    static void Main()
    {
        string fileName = "file.txt";
        string commitMessage = "Mass Commit, No:";

        if (Directory.Exists(".git"))
        {
            Console.WriteLine("Repository Initialised Already");
        }
        else
        {
            Console.WriteLine("We would recommend initialising a GitHub repository.");
            Console.WriteLine("Either use \"git initialise\" or use GitHub Desktop.");
            Console.WriteLine("If you would like us to do it for you, press y, otherwise press n");

            string choice = Console.ReadLine().ToLower();
            while (!(char.IsLetter(choice) && choice.Length == 1 && char.IsAscii(choice[0])))
            {
                Console.WriteLine("Please input a single letter. Either Y or N");
                choice = Console.ReadLine().ToLower();
            }

            if (choice == "y")
            {
                Console.WriteLine("Initialising Repository.");
                Process.Start("git", "init");
                File.WriteAllText("README.md", "# Commit");
                Process.Start("git", "add README.md");
                Process.Start("git", "commit -m \"Initial commit\"");
                Console.WriteLine("We have initialised the repository. Simply import it to GitHub to use this program");
                return;
            }
        }

        if (File.Exists(fileName))
        {
            Console.WriteLine("File Exists. Continuing");
        }
        else
        {
            File.WriteAllText(fileName, "File Initialised");
            Console.WriteLine("File created: " + fileName);
        }

        Console.Clear();
        Console.WriteLine("Mass Commits, By Kars.");
        Console.WriteLine("Please Input the amount of commits you would like. (Recommended 10-100 for speed)");

        string commitsInput = Console.ReadLine();
        while (!int.TryParse(commitsInput, out int commits))
        {
            Console.WriteLine("Please Input a Number");
            commitsInput = Console.ReadLine();
        }

        Console.WriteLine("Starting Mass Commits. Press Control + C at any time to stop the commit creation.");

        try
        {
            for (int i = 1; i < commits; i++)
            {
                using (StreamWriter file = new StreamWriter(fileName, true))
                {
                    file.WriteLine($"Edit {i}");
                }

                Process.Start("git", $"add {fileName}");
                Process.Start("git", $"commit -m \"{commitMessage} {i}\"");
            }

            Process.Start("git", "push origin master");
        }
        catch (SystemException)
        {
            Process.Start("git", "push origin master");
            Console.WriteLine($"Ended at {commits - 1} commits");
        }

        Console.WriteLine("Thank you for using my Commit thingy");
    }
}
