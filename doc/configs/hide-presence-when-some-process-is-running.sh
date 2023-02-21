#!/usr/bin/sh

while true; do
    cat << EOF
[ \
    { \
        "application_id": 0, \
        "state": "$(uname -r)", \
        "details": "$(uname -n)", \
        "large_image": { \
            "key": "some_image", \
            "text": null \
        }, \
        "small_image": null, \
        "start_timestamp": $(date -d "$(uptime -s)" +%s), \
        "end_timestamp": null, \
        "buttons": [ \
            { \
                "label": "some_button", \
                "url": "https://example.com/" \
            } \
        ], \
        "party": [1, 3] \
    } \
]
EOF

    sleep 10

    pids="$(pidof code) $(pidof wineserver)" # for example; you can put your own processes here

    if [ "${#pids}" != 0 ]; then
        echo "[]"

        for pid in $pids; do
            if [ -d "/proc/$pid" ]; then # if process haven't exited yet
                tail --pid=$pid -f /dev/null # wait for process to exit
            fi
        done
    fi
done
