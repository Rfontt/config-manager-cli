pub fn get_tools_registry() -> Vec<(String, Vec<String>)> {
    vec![
        (
            "aerospace".to_string(),
            vec!["~/.config/aerospace/aerospace.toml".to_string()],
        ),
        (
            "alacritty".to_string(),
            vec![
                "~/.config/alacritty/alacritty.toml".to_string(),
                "~/.config/alacritty/alacritty.yml".to_string(),
            ],
        ),
        (
            "bash".to_string(),
            vec![
                "~/.bashrc".to_string(),
                "~/.bash_profile".to_string(),
                "~/.bashenv".to_string(),
            ],
        ),
        (
            "zsh".to_string(),
            vec![
                "~/.zshrc".to_string(),
                "~/.zshenv".to_string(),
                "~/.zprofile".to_string(),
            ],
        ),
        (
            "neovim".to_string(),
            vec![
                "~/.config/nvim/init.lua".to_string(),
                "~/.config/nvim/init.vim".to_string(),
            ],
        ),
        (
            "vim".to_string(),
            vec!["~/.vimrc".to_string(), "~/.vim/vimrc".to_string()],
        ),
        ("git".to_string(), vec!["~/.gitconfig".to_string()]),
        (
            "gitignore".to_string(),
            vec!["~/.gitignore_global".to_string()],
        ),
        ("ssh".to_string(), vec!["~/.ssh/config".to_string()]),
        ("tmux".to_string(), vec!["~/.tmux.conf".to_string()]),
        (
            "homebrew".to_string(),
            vec!["~/.config/homebrew/brewfile".to_string()],
        ),
        (
            "espanso".to_string(),
            vec![
                "~/Library/Application Support/espanso/config/default.yml".to_string(),
                "~/Library/Application Support/espanso/config/default.yaml".to_string(),
            ],
        ),
    ]
}
