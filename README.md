This is a bash builtin thanks to [bash-builtins crate](https://docs.rs/bash-builtins/latest/bash_builtins/)

It provides some informations about the cpu affinity and the hostname, SLURM_JOB_ID, SLURM_PROCID and SLURM_LOCALID if they exists.


# Usage

Load the library into a bash builtin using with enable, use it an if needed remove it:

```
enable -f target/release/libcpubind_bash_builtin.so cpubind
cpubind
myhostname -  -  -  - cpu affinity: 0 1 2 3 4 5 6 7 8 9 10 11
enable -d cpubind
-bash: cpubind: command not found
```
