# wsl2-hosts
Windows app that patches the hosts file with the current wsl2 ip address.

## How to

### Create a config.txt
Create a config.txt next to wsl2-hosts.exe and add a line for each 
location you want to add to the hosts file.

For example:
```
localhost
wsl.local
```
Would result in something similar to this being added to your hosts file:
```
# WSL2(begin)
172.18.10.78            localhost
172.18.10.78            wsl.local
# WSL2(end)
```

### Run wsl2-hosts.exe
Run wsl2-hosts.exe 

If you have UAC enabled, you will be prompted to give the application 
administrator rights. This is required in order to edit the hosts file.

### Profit!
Your hosts file should now be altered.

## Help
The application checks if the ip used by wsl2 is already in your hosts file, 
otherwise it won't make any changes. You can check your current wsl2 
ip by running the following command in your powershell and look up the 
ip at eth0: `wsl -- ifconfig`
