set export := true

NO_COLOR := '\033[0m'
PURPLE := '\033[0;35m'

test day="--all":
    #!sh
    if cargo build; then
        cargo shuttle run &
        sleep 3
        echo -e "\n${PURPLE}Executing Tests${NO_COLOR}"
        cch23-validator ${day} --url http://127.0.0.1:8000
        kill $!
    fi

@watch:
    cargo watch -s "just test"
