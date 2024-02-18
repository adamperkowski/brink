#! /bin/bash

playerctl metadata mpris:artUrl | xargs curl -s | viu -w 50 -

