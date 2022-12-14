#!/usr/bin/python3

# Python 3 example
import sys
from rivescript import RiveScript

rs = RiveScript()
rs.load_directory("/opt/sam/scripts/rivescript/brain")
rs.sort_replies()

reply = rs.reply("localuser", sys.argv[1])
print(reply)

# vim:expandtab
