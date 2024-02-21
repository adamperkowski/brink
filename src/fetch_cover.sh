#! /bin/bash

clear
playerctl metadata mpris:artUrl | xargs curl -s | viu -w 50 -
echo
playerctl metadata title; playerctl metadata artist

