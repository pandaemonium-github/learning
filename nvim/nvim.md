
## Configure nvim

``` sh
git clone <path to nvim config git repo> "${XDG_CONFIG_HOME:-$HOME/.config}"/nvim
nvim --headless "+Lazy! sync" +qa
```
