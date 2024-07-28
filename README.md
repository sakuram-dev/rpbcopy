# rpbcopy

rpbcopy is a command-line tool that mimics the functionality of the `pbcopy` utility in Mac OS.  
It allows you to conveniently copy text from the command line to the clipboard.

## Installation

TBD


## Usage

To use rpbcopy, simply pipe the desired text to the `rpbcopy` command.

For example, to copy the text "Hello, world!" to your clipboard, run the following command:

```bash
echo "Hello, world!" | rpbcopy
```

You can also use rpbcopy to copy your SSH key to the clipboard. Run the following command:

```bash
cat ~/.ssh/id_rsa.pub | rpbcopy
```

For more information, run the following command:

```bash
rpbcopy --help
```

## License

rpbcopy is released under the [MIT License](https://opensource.org/licenses/MIT).


