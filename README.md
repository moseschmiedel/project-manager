# Project Manager

## Install

### cargo
### Nix (preferred)
Include following line in your flake based nix configuration:
```Nix
environment.systemPackages = [
  ...
  (builtins.getFlake "git+file:<path-to-project-manager-dir>/project-manager?rev=<git-ref>").packages.<your-system>.default
  ...
]
```

### Shell Integration
#### Bash
#### zsh
Add the following to your `.zshrc`:
```zsh
fpath=( $HOME/.config/zsh/functions "${fpath[@]}" )
autoload -Uz p
PROJECT_HOME="$HOME/projects"
```

Create `$HOME/.config/zsh/functions`:
```zsh
mkdir -p "$HOME/.config/zsh/functions"
```

And copy `integrations/zsh/p` to the `zsh/functions` directory:
```zsh
cp integrations/zsh/p $HOME/.config/zsh/functions/
```
