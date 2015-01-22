#!/usr/bin/env python

from urllib import urlopen
from bs4 import BeautifulSoup
from collections import OrderedDict
import re, json

print('Getting page...')
page = urlopen('http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html')
print('Parsing page...')
html = BeautifulSoup(page)

tables = html.find_all('table')[:2]
names  = ['opcodes', 'opcodes_cb']


for table, name in zip(tables, names):
    l = []
    for msb, row in enumerate(table.find_all('tr')[1:]):
        for lsb, data in enumerate(row.find_all('td')[1:]):

            opcode = '0x{:X}{:X}'.format(msb, lsb)
            try:
                mnemonic, bytes_and_cycles, flags = list(data.strings)
            except (ValueError):
                print('{} is not an opcode, skipping.'.format(opcode))
                continue

            mnemonic_parts = re.split(' |,', mnemonic)
            bytes, cycles = bytes_and_cycles.split(u'\xa0\xa0')

            l.append((opcode, 
                OrderedDict([
                    ('command', mnemonic_parts[0]),
                    ('args',    mnemonic_parts[1:]),
                    ('flags',   flags.split(' ')),
                    ('cycles',  map(int, reversed(cycles.split('/'))))
                    ])))

    print('Dumping '+name+'...')
    l.sort(key=lambda(k,v): (v['command'], k))
    json.dump(OrderedDict(l), open(name+'.json', 'w'), indent=2)
