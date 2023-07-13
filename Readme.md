## Git Switch

Switch between git user configs

### Installation

```
    cargo install git-switch
```

### Usage

Create a config file in home directory 

```
touch ~/.git-switch.json
```

Add configurations to the file

```
[
  {
    "host": "github.com",
    "username": "ashikmeerankutty",
    "email": "ashik9591@gmail.com"
  },
  {
    "host": "gitlab.com",
    "username": "ameerankutty",
    "email": "ameerankutty@gitlab.com"
  }
]
```

Inside the directory run git-switch. This will change the configs

### Using ZSH

Add the following code to `~/.zshrc` to auto change config on directory change

```
# Run git-switch on directory change
git_switch_on_chpwd() {
    git-switch
}

# Set up chpwd hook
add-zsh-hook chpwd git_switch_on_chpwd
```