${toc}

# References

. Become a bash scripting pro - full course (youtube) code is everything (https://www.youtube.com/watch?v=4ygaA_y1wvQ)


# Become a bash scriptingg pro

## the shebang

```sh
#!/usr/bin/env bash
```

tells the system which interpreter to use for the script.
prefer /usr/bin/env bash : more portable than /bin/bash

example:
```sh
#!/usr/bin/env bash

echo "Hello world!"
gsettings set org.gnome.desktop.interface color-scheme prefer-dark
kubectl get pods -A --no-headers | fzf | awk '{print $2, $1}' | xargs -n 2 sh -c 'kubectl describe pod $0 -n $1'
```

## permissions

```sh
chmod u+x ./hello.sh
```
give execute permission to the user

## variable 

```sh
name="Youtube"
```
important: no space between the variable name, the equal sign and the expression.
if there are some spaces, the interpreter will interpret the line as a command followed by arguments.


To reference a variable: $<variable name>
```sh
#!/usr/bin/env bash 
name="Youtube"
echo "Hello, $name"
```

If single quote is used instead of double quote, the variable expansion is not done.
```sh
echo 'Hello, $name'
```
Hello, $name

You can use the notation with curly braces ${} which has some nice capacities
```sh
echo "Hello, ${name}"
```
Hello, Youtube

```sh
echo "Hello, ${#name}"
```
Hello, 7


- Default value
```sh
#!/usr/bin/env bash

name=""
echo "Hello, ${name:-"Anonymous"}"
```
Hello, Anonymous

## Subshell

subshell: ()

```sh
> pwd
/home/pandaemonium/tmp
> (cd ..; pwd)
/home/pandaemonium
> pwd
/home/pandaemonium/tmp
```

use for command subsitution with $()

```sh
> var=$(pwd)
> echo $var
/home/pandaemonium
```

process substitution: <()

```sh
> diff <(ls ./v1) <(ls ./v2)
```

arithmetic expansion: $(())

```sh
> $((2+2))
4
```

## Command line arguments

```sh
> ./mergeav.sh input.mp4 input.wav output.mp4
       ($0)      ($1)      ($2)       ($3)
```
```
video_input=$1
audio_input=$2
output=$3

ffmpeg -i "$video_input" -i "$audio_input" -c:v copy - c:a aac 0:v:0 -map 1:a:0 "$output"
```

## If statement

```sh
if [[ some_condidion ]]; then
    echo "this condition is true"
elif [[ some_other_condition ]]; then
    echo "some other condition is true"
else
    echo "none of the condition are true"
fi
```

## Exit codes

exit code 0 -> Success :+1:\
any other value -> Failure :-1:

## Conditionals

- String comparison
```sh
val="a"
[[ "$val" == "a" ]] #equal
[[ "$val" != "b" ]] #not equal
```

- Numerical copon
```sh
num=1
[[ "$num" -eq 1 ]] # equal
[[ "$num" -ne 2 ]] # not equal
[[ "$num" -lt 2 ]] # less than
[[ "$num" -le 2 ]] # less than or equal 
[[ "$num" -gt 2 ]] # greater than
[[ "$num" -ge 2 ]] # greater than or equal
```

- Variable existence
```sh
val=""
[[ -z $val ]] # var is null (empty)
[[ -n $val ]] # var is not null
```

- File checks
```sh
file="./hello"
[[ -f $file ]] # file exists
[[ -d $file ]] # dir exists
[[ -e $file ]] # file or dir exists
```

- Permission checks
```sh
file="./hello"
[[ -r $file ]] # file is readable
[[ -w $file ]] # file is writable
[[ -x $file ]] # file is executable
```

- Combinations
1) Internal
```sh
[[ $val -gt 5 -a $val -lt 10 ]] # -a -> Logical AND
[[ $val -gt 5 -o $val -lt 3 ]] # -o -> Logical OR
```

2) External
```sh
[[ $val -gt 5 ]] && [[ $val -lt 10 ]] # Logical AND
[[ $val -gt 5 ]] || [[ $val -lt 3 ]] # Logical OR
```

## Useful commands

```sh
sleep 1 # wait 1 s
```

```sh
echo "What is your name?"
read -r name
echo "Hello, $name"
```

```sh 
read -p "Do you want to continue? (Y/n) " resp
if [[ $resp != "Y" ]]; then
    exit 1
fi
echo "Continuing..."
```

## Good behavior

```sh
# make bash more sane
set -euo pipefail
```

```sh
set -e          # exit on error
set -u          # exit on unset variable
set -o pipefail # exit on pipe fail
```

## Conditional execution

- command1 && command2  # If the first succeeds, perform the next
- command1 && command2  # If the first fails, perform the next

## Arrays

```sh
my_arr=(1 2 3 4 5)
echo ${my_arr[0]} # 1
echo ${my_arr[@]} # @ refers to all elements (1 2 3 4 5)
echo ${#my_arr[@]} # prints length of arrray (5)
```

## For loop

```sh 
my_arr=(1 2 3 4 5)
for item in $my_arr[@]; do 
    echo $item
done
```

```sh
# C style
for (( i = 0; i < 10; i++ )); do 
    echo "$i"
done
```
```sh
# range
for i in {1..10}; do
    echo "$i"
done
```
```sh
# patttern matching
for item in ./content/*.md; do
    echo "$item"
done
```
```sh
# command result
for item in $(ls); do
    echo "$item"
done
```

## While loop

```sh
counter=0
while [[ $counter -lt 5 ]]; do
    echo $counter
    ((counter++))
done
```

```sh
# wait unti true
pod_name="myyapp"
namespace="default"

while true; do
    status=$(kubectl get pod $pod_name -n $namespace --no-headers -o custom-columns=":status.phase")
    if [[ "$status" == "Running" ]]; then 
        break
    else
        echo "Waiting for pod $pod_name to get into a running state."
        sleep 1
    fi
done
echo "Pod $pod_name is running."
```

## Breaking out

```sh
echo "Provisioning environment"
./instance.sh
./dns.sh
echo "Provisioning complete!"
```

- sourcing file

```sh
# script1.sh
. ./script2.sh
```
include the text of scipt2.sh in-place in script1.sh
#

## Functions

```sh
function ensure_dependency () {
    local tool=$1   # arg1 of function  (local to scope variable in function)
    local install_url=$2
    if ! command -v "$tool" &> /dev/null; then 
        echo "Error: $tool is not installed." >&2
        echo "Please install $tool from $install_url" >&2
        exit 1
    fi
}
```

## Temp and cleanup

```sh
tempfile=$(mktemp)
trap "rm -f $tempfile" EXIT     # On signal exit, remove temp file (explicit)
echo "Hello, Youtube!" > $tempfile
```

```sh 
tempdir=$(mktemp -d)
trap "rm -rf $tempdir" EXIT
echo "Hello, Youtube" > "$tempdir/hello"
```


