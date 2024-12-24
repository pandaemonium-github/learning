nmcli nm wifi off
nmcli nm wifi on

rfkill block all
rfkill unblock all

if=eth0
sudo ip link set $if down

sudo ip link set enx98bb1e1d8941 down

