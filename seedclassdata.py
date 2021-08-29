import requests
import json
import copy
import urllib.parse

# TODO: Add more jobs eventually
JOBS = ['PLD']
XIVAPI_SEARCH_URL = 'https://xivapi.com/search?'
SEARCH_FILTERS = [
    'IsPvP=0,ActionCategory.ID>=2',
    'ActionCategory.ID<=4',
    'IsPlayerAction=1'
]
# TODO: Will need more columns than just this, like cast times.
# NOTE: The DescriptionJson has conditionals for player levels, but will omit some
# resource gain information.
COLUMNS = ['ID', 'Name', 'Icon', 'Description']
SEARCH_VARS = {
    'indexes': 'Action',
    'columns': ','.join(COLUMNS)
}


def class_job_category_filter(job):
    return 'ClassJobCategory.%s=1' % job


def search_vars(job):
    vars = copy.deepcopy(SEARCH_VARS)
    filters = SEARCH_FILTERS + [class_job_category_filter(job)]
    vars['filters'] = ','.join(filters)
    return vars


for job in JOBS:
    request_url = XIVAPI_SEARCH_URL + urllib.parse.urlencode(search_vars(job))
    print('Requesting %s' % request_url)
    response = requests.get(request_url)
    data = response.json()['Results']
    with open('app/data/' + '%s.json' % job, 'w') as outfile:
        json.dump(data, outfile)
