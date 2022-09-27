#!bin/bash

# build the project static files
cd ../nitride-ui &&  npm run build
cd ../nitride
# copy the static files to the server
rm -rf views/*
cp -r ../nitride-ui/dist/* ./views