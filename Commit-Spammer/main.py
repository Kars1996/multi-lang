import os
import subprocess

file_name = 'file.txt'  
commit_message = 'Mass Commit, No:'

if os.path.isdir('.git'):
    print('Repository Initialised Already')
else:
    print('We would reccomend initialising a github repository.')
    print('Either use "git initialise" or use github desktop.')
    print('If you would like us to do it for you press y, otherwise press n')
    choice = input('> ').lower()
    while not choice.isalpha() or not choice.isascii() or len(choice) != 1:
        print('Please input a single letter. Either Y or N')
        choice = input('> ').lower()
    if choice == 'y':
        print('Initialising Repository.')
        subprocess.run(['git', 'init'])
        with open('README.md', 'w') as file:
            file.write('# Commit')
        subprocess.run(['git', 'add', 'README.md'])
        subprocess.run(['git', 'commit', '-m', 'Initial commit'])
        print('We have initilaised the repository, Simply import it to github to use this program')
        exit()
        

if os.path.exists(file_name):
    print('File Exists. Continuing')
else:
    with open(file_name, 'w') as file:
        file.write("File Initialised")
    print("File created: ", file_name)

os.system('cls')
print('Mass Commits, By Kars.')
print('Please Input the ammount of commits you would like. (Reccomended 10-100 for speed)')
commits = input('> ')
while not commits.isdigit():
    print('Please Input a Number')
    commits = input('> ')
print('Starting Mass Commits. Press Control + C at any time to stop the commit creation.')
commits = int(commits)

try:
    for i in range(1, commits):
        with open(file_name, 'a') as file:
            file.write(f'Edit {i}\n')
        subprocess.run(['git', 'add', file_name])
        subprocess.run(['git', 'commit', '-m', f'{commit_message} {i}'])
    subprocess.run(['git', 'push', 'origin', 'master'])
except KeyboardInterrupt:
    subprocess.run(['git', 'push', 'origin', 'master'])
    print(f'Ended at {i} commits')
print('Thank you for using my Commit thingy')

