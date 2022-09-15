#!bin/bash

# build the project static files
cd ../dashboard &&  npm run build
cd ../nitride
# copy the static files to the server
mkdir views
cp -r ../dashboard/dist/* ./views