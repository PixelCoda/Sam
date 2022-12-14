#!/bin/bash
# Ask the user for their name
echo "PG_DBNAME?"
read PG_DBNAME

echo "PG_USER?"
read PG_USER

echo "PG_PASS?"
read PG_PASS

echo "PG_ADDRESS?"
read PG_ADDRESS


echo "export PG_DBNAME=$PG_DBNAME"
echo "export PG_USER=$PG_USER"
echo "export PG_PASS=$PG_PASS"
echo "export PG_ADDRESS=$PG_ADDRESS"