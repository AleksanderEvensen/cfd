# Change Fuzzy Directory

A fuzzy finder alternative to the classic `cd` command

This program doesn't change the current working directory, and needs to be paired up with a shell script

PowerShell:
```ps
function cfd {
	param ($dst)
	$path = ./path/to/cfd/binary.exe $dst
	cd $path
}
```


![Program somewhat in action](images/showcase.gif)

