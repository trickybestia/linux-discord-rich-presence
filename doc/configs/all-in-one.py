#!/usr/bin/python3

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
        'state': cmd('uname -r'),   # Use `cmd('command_to_run')` if you want to run command like in terminal and show it's result, otherwise if you want show text here comment this line andd uncomment next:
        # 'state': 'Your text',
        'details': cmd('uname -n'), # same here 
        'large_image': {'key': 'some_image', 'text': None},
        'small_image': None,
        'start_timestamp': int(cmd('date -d "$(uptime -s)" +%s')),
        'end_timestamp': None,
        'buttons': [{'label': 'some_button',
                    'url': 'https://example.com/'
                     }],
        'party': [1, 3], # 'party': [current party size, max party size],
    }]

while True:
    print(json.dumps(update()))

    stdout.flush()

    sleep(10)
