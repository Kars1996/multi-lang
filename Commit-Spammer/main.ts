import { execSync } from 'child_process';
import * as fs from 'fs';

const file_name = 'file.txt';
const commit_message = 'Mass Commit, No:';

if (fs.existsSync('.git')) {
  console.log('Repository Initialised Already');
} else {
  console.log('We would recommend initialising a GitHub repository.');
  console.log('Either use "git initialise" or use GitHub Desktop.');
  console.log('If you would like us to do it for you press y, otherwise press n');
  let choice: string = ''; 
  while (!choice.match(/^[yn]$/)) {
    choice = prompt('> ')?.toLowerCase() || '';
  }
  if (choice === 'y') {
    console.log('Initialising Repository.');
    execSync('git init');
    fs.writeFileSync('README.md', '# Commit');
    execSync('git add README.md');
    execSync('git commit -m "Initial commit"');
    console.log('We have initialised the repository. Simply import it to GitHub to use this program.');
    process.exit(0);
  }
}

if (fs.existsSync(file_name)) {
  console.log('File Exists. Continuing');
} else {
  fs.writeFileSync(file_name, 'File Initialised');
  console.log(`File created: ${file_name}`);
}

console.clear();
console.log('Mass Commits, By Kars.');
console.log('Please Input the amount of commits you would like. (Recommended 10-100 for speed)');

let commits: number = 0;
while (isNaN(commits) || commits <= 0) {
  const input = prompt('> ');
  commits = parseInt(input || '', 10);
  if (isNaN(commits) || commits <= 0) {
    console.log('Please Input a valid number');
  }
}

console.log('Starting Mass Commits. Press Control + C at any time to stop the commit creation.');

try {
  for (let i = 1; i < commits; i++) {
    fs.appendFileSync(file_name, `Edit ${i}\n`);
    execSync(`git add ${file_name}`);
    execSync(`git commit -m "${commit_message} ${i}"`);
  }
  execSync('git push origin master');
} catch (error) {
  execSync('git push origin master');
  console.log(`Ended at ${commits - 1} commits`);
}

console.log('Thank you for using my Commit thingy');
