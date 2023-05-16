**README: Diceware Installation and Usage**

Follow these steps to install and use Diceware:

Note: Whenever you see commands, do not include the backticks (single or triple).

**Step 1**: Open the terminal at the desired directory by selecting the directory and choosing "New Terminal at Folder".

**Step 2**: Run the command below (only what is between the ```), and enter your password when prompted:

```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" && echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.profile && eval "$(/opt/homebrew/bin/brew shellenv)" && brew install node && chmod +x diceware.js && ./diceware.js
```

Note: If it doesn't work the first time, close the terminal and repeat steps 1 and 2.

Note: It should immediately run the script after you're done.

**Step 3**: There are two options to run the script thereafter:
- Option 3a: Use the terminal at the current directory and run: `./diceware.js`
- Option 3b: Double click on `diceware.js`

**Step 4**: Adding a custom logo:
- Step 4a: Copy the `diceware-logo.png` file.
- Step 4b: Right-click on the `diceware.js` file and select 'Get Info'.
- Step 4c: Click on the logo image at the top and then press `cmd+v` to paste the new logo.

**Step 5 (Optional)**: You can create an alias or shortcut to the `diceware.js` file and place it anywhere you prefer, such as your desktop or applications folder.