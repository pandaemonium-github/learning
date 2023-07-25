
## How to remove the green background on directories for ls commands

add to rc file (.bashrc, or .zshrc ...)
```sh
LS_COLORS=$LS_COLORS:'di=0;35:' ; export LS_COLORS
```

[How do I change the color for directories with ls in the console?](https://askubuntu.com/questions/466198/how-do-i-change-the-color-for-directories-with-ls-in-the-console)