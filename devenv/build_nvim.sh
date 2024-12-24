sudo apt-update
sudo apt-install make gettext ninja-build

git clone https://github.com/neovim/neovim --depth 1
cd neovim && make CMAKE_BUILD_TYPE=RelWithDebInfo
#install to /usr/local by default ( to change set CMAKE_INSTALL_PREFIX)
sudo make install
# it is possible on debian/ubuntu to create deb-package and install it
# it helps ensure cle
# an removal of installed files
