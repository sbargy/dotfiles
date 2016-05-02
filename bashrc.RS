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
alias l='less'
alias m='more'
alias gt='cd /app/tools;ls'
alias gR='cd /app/tools/rps/lib/RPS/;ls'
alias gB='cd /app/tools/bookpub/lib/BookPub/;ls'
alias gi='cd /app/tools/bookpub/lib/BookPub/Import/Importer;ls'
alias gri='cd /app/tools/rps/lib/RPS/Import/;ls'
alias gs='cd /app/tools/sale_import/etc/bookpub_sample_sales_files;ls'
alias grs='cd /app/tools/sale_import/etc/sample_sales_files;ls'
alias swapd='pushd +1'

export HISTSIZE=1000000
export HISTCONTROL=ignorespace
export CVSROOT=:pserver:bargabus@atlas:/cvsroot/tools

export PATH=$PATH:$HOME/.gems/bin:/app/tools/common/bin:.

export PAGER=/usr/bin/less
export EDITOR=/usr/bin/vi

PS1='\n[\w]\n\u@\h\$ '

export MYSQL_PS1="[`hostname`:\d]> "
#export MYSQL_PS1="[\h:\d]> "
export LESS="-S"

if [ "$TERM" == "screen" ]
then

setWindowTitle() {
    echo -ne "\e]2;$*\a"
}

updateWindowTitle() {
    setWindowTitle "${HOSTNAME%%.*}:${PWD/$HOME/~}"
}

PROMPT_COMMAND=updateWindowTitle
export -f updateWindowTitle

fi

tty -s && echo -ne "\n\n"
tty -s && switch_branch -w
tty -s && echo -ne "\n\n"
tty -s && uptime
