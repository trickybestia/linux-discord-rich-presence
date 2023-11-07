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

    if [ ! -z "${pids// }" ]; then
        echo "[]"

        for pid in $pids; do
            tail --pid=$pid -f /dev/null # wait for process to exit
        done
    fi
done
