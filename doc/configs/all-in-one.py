#!/bin/python

import json
from os import popen
from contextlib import suppress

def cmd(command):
    with popen(command) as process:
        return process.read()[0:-1]

def update():
    return {'update_delay': 10, 'items': [{
        'application_id': 000000000000000000,
        'state': cmd('uname -r'),
        'details': cmd('uname -n'),
        'large_image': {'key': 'some_image', 'text': None},
        'small_image': None,
        'start_timestamp': int(cmd('date -d "$(uptime -s)" +%s')),
        'end_timestamp': None,
        'buttons': [{'label': 'some_button',
                    'url': 'https://example.com/'
                     }],
    }]}

with suppress(EOFError):
    while True:
        if input() == 'update':
            print(json.dumps(update()))
