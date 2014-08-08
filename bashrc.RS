# .bashrc

# User specific aliases and functions

# Source global definitions
if [ -f /etc/bashrc ]; then
        . /etc/bashrc
fi

umask 0002
set -o vi

#alias ls='ls -hFC --color=tty'                # classify files in colour
alias ls='ls -hFC'
alias ll='ls -l'                              # long list
alias la='ls -A'                              # all but . and ..
alias l='ls -CF'                              #
alias m='more'
alias gt='cd /app/tools;ls'
alias gi='cd /app/tools/bookpub/lib/BookPub/Import/Importer;ls'
alias gri='cd /app/tools/rps/lib/RPS/Import/;ls'
alias gs='cd /app/tools/sale_import/etc/bookpub_sample_sales_files;ls'
alias grs='cd /app/tools/sale_import/etc/sample_sales_files;ls'
alias swapd='pushd +1'

export HISTSIZE=1000000
export CVSROOT=:pserver:bargabus@atlas:/cvsroot/tools

export PATH=$PATH:$HOME/.gems/bin:/app/tools/common/bin:.

export GEM_HOME=$HOME/.gems
export GEM_PATH=$HOME/.gems:/var/lib/gems/1.8/
export RUBYOPT=rubygems

export PAGER=/usr/bin/less

PS1='\n[\w]\n\u@\h\$ '

export MYSQL_PS1="[`hostname`:\d]> "
#export MYSQL_PS1="[\h:\d]> "
export LESS="-S"

if [ "$TERM" == "terminator" ]
then

setWindowTitle() {
    echo -ne "\e]2;$*\a"
}

updateWindowTitle() {
    setWindowTitle "${HOSTNAME%%.*}:${PWD/$HOME/~}"
}

PROMPT_COMMAND=updateWindowTitle

fi

tty -s && echo -ne "\n\n"
tty -s && switch_branch -w
tty -s && echo -ne "\n\n"
tty -s && uptime

