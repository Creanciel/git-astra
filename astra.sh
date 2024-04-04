#!/bin/bash -e

THIS_DIRECTORY="$(cd $(dirname $0); pwd)"

INSTALL_DIRECTORY=/usr/bin
INSTALL_PATH="$INSTALL_DIRECTORY/git-astra"

build() {
    cargo build --release
    echo "Built git-astra"
    echo "Install by \`./astra.sh install\`"
}

install(){
    if [ ! ${EUID:-${UID}} = 0 ]
    then
        echo "Excute as root"
        echo "Abort"
        exit 1
    fi

    cd "$THIS_DIRECTORY"
    cp ./target/release/git-astra "$INSTALL_DIRECTORY"
    echo "Succesed"
    echo 'Use `git astra git@~` instead of `git clone git@~`.'
}

uninstall(){
    if [ ! ${EUID:-${UID}} = 0 ]
    then
        echo "Excute as root"
        echo "Abort"
        exit 1
    fi

    rm -f "$INSTALL_PATH"
    echo "Uninstalled \"$INSTALL_PATH\""
}

help() {
    cat <<TEXT
   ______   ____  ______       ___     _____  ______  ____     ___
  / ____/  /  _/ /_  __/      /   |   / ___/ /_  __/ / __ \   /   |
 / / __    / /    / /        / /| |   \__ \   / /   / /_/ /  / /| |
/ /_/ /  _/ /    / /        / ___ |  ___/ /  / /   / _, _/  / ___ |
\____/  /___/   /_/        /_/  |_| /____/  /_/   /_/ |_|  /_/  |_|

                git clone wrapper for multi accounts.


    1. ./astra.sh build
    2. ./astra.sh install (root required)

    x. ./astra.sh uninstall (root required)
TEXT
}

execute() {
    case "$1" in
        build)
            build
            ;;

        install)
            install
            ;;

        uninstall)
            uninstall
            ;;

        *)
            help
            ;;
    esac
}

execute "$@"
