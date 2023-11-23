const { execSync } = require('child_process');
const fs = require('fs');

const fileName = 'file.txt';
const commitMessage = 'Mass Commit, No:';

if (fs.existsSync('.git')) {
  console.log('Repository Initialized Already');
} else {
  console.log('We would recommend initializing a GitHub repository.');
  console.log('Either use "git initialise" or use GitHub Desktop.');
  console.log('If you would like us to do it for you press y, otherwise press n');

  const readline = require('readline-sync');
  let choice;

  do {
    choice = readline.question('> ').toLowerCase();
  } while (!choice.match(/[yn]/));

  if (choice === 'y') {
    console.log('Initializing Repository.');
    execSync('git init');
    fs.writeFileSync('README.md', '# Commit');
    execSync('git add README.md');
    execSync('git commit -m "Initial commit"');
    console.log('Repository initialized. Simply import it to GitHub to use this program.');
    process.exit();
  }
}

if (fs.existsSync(fileName)) {
  console.log('File Exists. Continuing');
} else {
  fs.writeFileSync(fileName, 'File Initialized');
  console.log(`File created: ${fileName}`);
}

console.clear();
console.log('Mass Commits, By Kars.');
console.log('Please Input the amount of commits you would like. (Recommended 10-100 for speed)');

let commits;

do {
  commits = parseInt(readline.question('> '), 10);
} while (isNaN(commits) || commits <= 0);

console.log('Starting Mass Commits. Press Control + C at any time to stop the commit creation.');

try {
  for (let i = 1; i < commits; i++) {
    fs.appendFileSync(fileName, `Edit ${i}\n`);
    execSync(`git add ${fileName}`);
    execSync(`git commit -m "${commitMessage} ${i}"`);
  }

  execSync('git push origin master');
} catch (error) {
  execSync('git push origin master');
  console.log(`Ended at ${commits - 1} commits`);
}

console.log('Thank you for using my Commit thingy');
