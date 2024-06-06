# Git Switch

Switch between Git user configs

## Installation

```bash
cargo install git-switch
```

To install `git-switch`, use the above command. This assumes you have Rust and Cargo installed on your system.

## Usage

1. Create a configuration file in the home directory. Run the following command:

```bash
touch ~/.git-switch.json
```

This will create a file named `.git-switch.json` in your home directory (`~`).

2. Open the `.git-switch.json` file in a text editor and add the desired Git configurations. The file should be in JSON format and contain an array of objects. Each object represents a Git host (e.g., GitHub, GitLab) and includes the following properties:
   - `"host"`: The hostname of the Git service.
   - `"username"`: The desired username.
   - `"email"`: The desired email address.

Here's an example configuration file:

```json
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

Feel free to add or remove configurations based on your needs.

3. To switch the Git configurations, navigate to the desired directory in your terminal and run the following command:

```bash
git-switch
```

This will read the `.git-switch.json` file from your home directory and update the Git configurations based on the current directory.

## Using ZSH

To automatically change the Git config when the directory changes, follow these steps:

1. Open your Zsh configuration file. Run the following command:

```bash
code ~/.zshrc
```

Replace `code` with the command for your preferred text editor.

2. Add the following code to the `~/.zshrc` file:

```bash
# Run git-switch on directory change
git_switch_on_chpwd() {
    git-switch
}

# Set up chpwd hook
add-zsh-hook chpwd git_switch_on_chpwd
```

This code sets up a `chpwd` hook, which is executed whenever the current directory changes. It calls `git-switch` to update the Git configurations accordingly.

3. Save the changes and reload your Zsh configuration:

```bash
source ~/.zshrc
```

Now, whenever you change directories in your terminal, the `git-switch` command will automatically be executed to update the Git configurations based on the current directory.
