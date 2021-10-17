#!/bin/bash

function print_fat() {
  msg=$1
  bold=$(tput bold)
  normal=$(tput sgr0)
  echo -e "${bold}${msg}${normal}"
}

function exec_and_echo() {
  cmd=$1
  print_fat "[+] Executing \`${cmd}\`."
  eval "$cmd"
}

