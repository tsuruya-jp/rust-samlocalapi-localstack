#!/bin/bash

key_name="user_id"
table_name="users"

aws --endpoint-url=http://localstack:4566 dynamodb create-table \
    --table-name=$table_name \
    --attribute-definitions AttributeName=$key_name,AttributeType=S \
    --key-schema AttributeName=$key_name,KeyType=HASH \
    --billing-mode PAY_PER_REQUEST