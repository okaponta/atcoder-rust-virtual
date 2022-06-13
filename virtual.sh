#!/bin/bash

# Usage
# $1 -> name of virtual contest
# ./config -> num path

echo "SETUP FOR VIRTUAL CONTEST $1"
cp -r ./template ./$1

# Cargo.tomlのnameを$1にかえる
sed -i '' -e "2 s/template/$1/" ./$1/Cargo.toml

# config から1行ずつ 番号 URL を読み込む
cat ./config | while read num url; do
    echo "$num"
    echo "$url"
    ## compete.tomlを書き換え
    sed -i '.bk' -e "134 s|target|$url|" ./compete.toml
    sed -i '' -e "135 s/target/$num/" ./compete.toml
    ## cd $1 して、cargo compete add
    cd $1
    cargo compete add
    cd ..
    cp ./compete.toml.bk ./compete.toml
done

LINE=$(wc -l ./$1/Cargo.toml | awk '{print $1}')
echo $LINE
NAME=$(expr $LINE - 44)
echo $NAME

# 最初に登録したdummyケースを消す
