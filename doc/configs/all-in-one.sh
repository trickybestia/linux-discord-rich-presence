#!/usr/bin/sh

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
