
## display graphic cards information

```sh
sudo lshw -C display
```


```sh
lspci | grep ' VGA ' | cut -d" " -f 1 | xargs -i lspci -v -s {}
```

```sh
nvidia-smi
```

## install nvidia drivers

from ppa:

```
sudo add-apt-repository ppa:graphics-drivers/ppa
ubuntu-drivers devices
sudo ubuntu-drivers autoinstall
sudo reboot
```

(Optional) Mark the driver as hold to prevent auto-upgrading (since it is a server):
```
dpkg-query -W --showformat='${Package} ${Status}\n' | grep -v deinstall | awk '{ print $1 }' | \
    grep -E 'nvidia.*-[0-9]+$' | \
    xargs -r -L 1 sudo apt-mark hold
```


#uninstall nvidia drivers

```
sudo apt-get remove --purge '^nvidia-.*'
sudo apt-get remove --purge '^libnvidia-.*' 
sudo apt-get remove --purge '^cuda-.*'
```
