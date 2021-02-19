# rust-minigrep

This is a command line tool that interacts with file and command line input/output.
The rust-minigrep is a version of the classic command line tool grep(globally searcha regular expression and print.

## How it works

It searches a specified file for a specified string. To do so, it takes as its arguments a filename and a string.
Then it reads the file, finds lines in that file that contain the string argument and prints those lines.

It can also read the value of an environment variable to allow the user to configure the behaviour of the tool
Prints error messages to the standard error console stream(stderr) instead of standard output
