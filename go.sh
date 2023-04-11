#!/bin/bash

clear

filever="./vernum"
let VERNUM=`cat ${filever}`+1
echo ${VERNUM} > ${filever}
VERNUM="`date +"%Y-%m-%d %H:%M:%S"` version=${VERNUM}"
echo ${VERNUM}

name=wasm-ws

#HOST=/home/work/www/rustcode/${name}
#SITE=http://q.lleo.me/rustcode/${name}/
SITE=http://127.0.0.1/rustcode/${name}/

#rm pkg/* ; rm pkg/.gitignore

wasm-pack build --target web --out-name wasm

HOST=./www
rm ${HOST}/*.js
rm ${HOST}/*.html
rm ${HOST}/*.css
rm ${HOST}/*.wasm

cp -f pkg/wasm_bg.wasm 	${HOST}/wasm.wasm
cp -f pkg/wasm.js      	${HOST}
cp -f ./index.html 	${HOST}
sed -i "s#///DATA///#${VERNUM}#g" ${HOST}/index.html
cp -f ./main.js    	${HOST}
cp -f ./main.css   	${HOST}

echo
echo "Open: ${SITE}"
echo

exit

#=========================================
HOST="ll"
if [ ! -d "${HOST}" ]; then mkdir ${HOST} ; fi
cp -f pkg/wasm_bg.wasm ${HOST}/wasm.wasm
cp -f pkg/wasm.js      ${HOST}/wasm.js
cp -f ./index.html        ${HOST}
sed -i "s#/rustcode/wasm-ws/#/${HOST}/#g" ${HOST}/index.html
sed -i "s#///DATA///#${VERNUM}#g" ${HOST}/index.html
cp -f ./main.js           ${HOST}
cp -f ./main.css          ${HOST}
rsync --rsh='ssh -p123' -avP ./${HOST}/* lleo@q.lleo.me:/home/work/www/${HOST}/
#=========================================

