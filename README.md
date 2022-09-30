# Change Fuzzy Directory

A fuzzy finder alternative to the classic `cd` command

![CFD in action](images/showcase.gif)

This standalone exe can't change the current working directory, and needs to be paired up with a shell script

PowerShell:

```PowerShell
function cfd {
	# Run Executable
	$result = . "./path/to/cfd/binary.exe" $args # Or "cfd" if cfd is in your path

	# Check for result
	if ($result) {

		# Set Location to the returned value
		Set-Location $result
	}
}
```

Bash (.bashrc) / Zsh (.zshrc)

```sh
cfd() {
    local result=$(./path/to/cfd/binary "$@") # Or "command cfd" if cfd is in your path
    [ -n "$result" ] && cd -- "$result"
}
```
