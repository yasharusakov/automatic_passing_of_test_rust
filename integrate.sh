#!/bin/bash

# This file needs to "intergrate" apot to Linux system. It means make avaible apot from whenever place in system and creates .desktop

path_to_bin_file="./src-tauri/target/release/automatic-passing-of-test-rust"

if [[ -e "$path_to_bin_file" ]]; then
    sudo ln -s "$PWD"/"$path_to_bin_file" /usr/local/bin/apot_r

    cat << EOF > ~/Desktop/apot_r.desktop
[Desktop Entry]
Encoding=UTF-8
Version=1.0
Type=Application
Terminal=false
Exec=/usr/local/bin/apot_r
Name=Automatic Passing Of Test
EOF
else
    echo "Error! Check have you built project. See in README file 'Development and Release'"
fi 
