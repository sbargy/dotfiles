parse_git_branch() {
	git branch 2> /dev/null | sed -e '/^[^*]/d' -e 's/* \(.*\)/(\1)/'
}

datediff() {
    date --version >> /dev/null 2>&1
    RETVAL=$?
    if [ -e /usr/local/bin/gdate ] && [ $RETVAL -ne 0 ] ;
    then 
        DATEBIN=/usr/local/bin/gdate
    else
        DATEBIN=`which date`
    fi

    d1=$($DATEBIN -d "$1" +%s)
    d2=$($DATEBIN -d "$2" +%s)
    echo $(( (d1 - d2) / 86400 )) days
}
