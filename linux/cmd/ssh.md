
## Upload public key to the remote server to avoid typing password

```sh 
ssh-copy-id -i ~/.ssh/id_rsa.pub remote-user@remote-host
```

## Create ssh jump to access host not directly accessible

```sh
ssh -A -J user@jump-server user@destination-server
```

## Forward port to connect to gdbserver

```sh
ssh -L 1234:localhost:1234 user@remote-host
```

start gdb server, and attach to process pid to debug:
```sh
gdbserver --attach 0.0.0.0:1234 <pid>
```
