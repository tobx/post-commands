#!/usr/bin/env bash

url="http://127.0.0.1:2180/execute"
header="Content-Type: application/json"

data="{
    \"name\": \"test\",
    \"args\": [\"arg1\", \"arg2\"]
}"

curl --verbose --header "${header}" --data "${data}" "${url}"
