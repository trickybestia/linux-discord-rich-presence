#!/bin/python

import json
from os import popen
from sys import stdout
from time import sleep

def cmd(command):
    with popen(command) as process:
        return process.read()[0:-1]

# Missing items behave as if they were None
def update():
    return [{
        'application_id': 000000000000000000,
        'state': cmd('uname -r'),   # If you want text here to show remove the cmd and brackets also -r or -n
        'details': cmd('uname -n'), # same here 
        'large_image': {'key': 'some_image', 'text': None},
        'small_image': None,
        'start_timestamp': int(cmd('date -d "$(uptime -s)" +%s')),
        'end_timestamp': None,
        'buttons': [{'label': 'some_button',
                    'url': 'https://example.com/'
                     }],
    }]

while True:
    print(json.dumps(update()))

    stdout.flush()

    sleep(10)
