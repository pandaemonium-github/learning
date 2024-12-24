# Build docker container for neovim

## Alpine

Dependencies required for building nvim from sources found from: https://medium.com/@be_likedeep/building-neovim-from-source-4a5587e4b4bf

If gettext is missing from installed depencies, there is an error raised during compilation of neovim, indicating missing libintl.

https://github.com/fxdgear/alpine-neovim/blob/master


mount docker socket to access docker from docker:
docker run -it -v /:/data -v /run/user/1000/docker.sock:/var/run/docker.sock:rw nvim sh