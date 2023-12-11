set export := true

NO_COLOR := '\033[0m'
PURPLE := '\033[0;35m'

test:
    #!sh
    if cargo build; then
        cargo shuttle run &
        sleep 3
        echo -e "\n${PURPLE}Executing Tests${NO_COLOR}"
        cch23-validator --all
        kill $!
    fi

@watch:
    cargo watch -s "just test"
