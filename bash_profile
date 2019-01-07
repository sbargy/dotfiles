[ -r ~/.bash_aliases ] && source ~/.bash_aliases
[ -r ~/.bash_functions ] && source ~/.bash_functions

# Set architecture flags
export ARCHFLAGS="-arch x86_64"
# Ensure user-installed binaries take precedence
export PATH=/usr/local/bin:/usr/local/sbin:$PATH

#PS1='\h:\u:\w\$ '
#export PS1='\n[\w]\n\u@\h\$ '
export PS1="\n[\w] \$(parse_git_branch)\n\u@\h\$ "

alias ls='ls -FC'
alias ll='ls -l'
alias la='ls -a'
alias lh='ls -lh'
alias m='more'
alias l='less'

export PATH=$PATH:~/bin:.
export PATH=/usr/local/share/npm/bin:$PATH
export ROOTDIR=/Users/scott
export PAGER=/usr/bin/less
export EDITOR=/usr/bin/vim
export LESS="-SR"

alias histsum='cut -f1 -d" " ~/.bash_history | sort | uniq -c | sort -nr | head -n 30'
alias gvim='open -n -a MacVim.app '

export HISTSIZE=500000
export HISTCONTROL=ignorespace
export LANG=en_US.UTF-8
export NODE_PATH='/usr/local/lib/node'
export PKG_CONFIG_PATH=/opt/X11/lib/pkgconfig/
export MYSQL_PS1="[`hostname`:\d]> "

set -o vi

if [ -f $(brew --prefix)/etc/bash_completion ]; then
  . $(brew --prefix)/etc/bash_completion
fi

# rbenv
export PATH="$HOME/.rbenv/bin:$PATH"
eval "$(rbenv init -)"

# this fixes a broken kqueue, https://github.com/tmux/tmux/issues/475
export EVENT_NOKQUEUE=1

# npm
NPM_PACKAGES="${HOME}/.npm-packages"
PATH="$NPM_PACKAGES/bin:$PATH"
export MANPATH="$NPM_PACKAGES/share/man:$(manpath)"
export NODE_PATH=$NODE_PATH:$HOME/.npm-packages/lib/node_modules

export ANSIBLE_NOCOWS=1

# python virtualenv
#export WORKON_HOME=~/.virtualenvs
#export VIRTUALENVWRAPPER_PYTHON="/usr/local/Cellar/python/2.7.14/bin/python2.7"
#export VIRTUALENVWRAPPER_VIRTUALENV="/usr/local/bin/virtualenv"
#source /usr/local/bin/virtualenvwrapper.sh
