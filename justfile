set export := true

NO_COLOR := '\033[0m'
PURPLE := '\033[0;35m'

test:
    #!sh
    if cargo build; then
        cargo shuttle run > /dev/null &
        sleep 2
        echo -e "\n${PURPLE}Executing HURL Tests${NO_COLOR}"
        hurl --continue-on-error --color --test --error-format long ./tests/*.hurl
        kill $!
    fi

@watch:
    cargo watch -s "just test"
