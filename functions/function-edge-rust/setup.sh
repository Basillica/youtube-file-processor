#!/bin/sh
set -e

function_name="utilityfunc"
rg=""
stacc=""
location="westeurope"

if ! az storage account create --name $stacc --location $location --resource-group $rg --sku Standard_LRS; then
  echo "Error: Failed to create Azure Function storeage account."
  exit 1
fi

if ! az functionapp create --resource-group $rg --consumption-plan-location westeurope --runtime custom --functions-version 4 --name $function_name --os-type linux --storage-account $stacc; then
  echo "Error: Failed to create Azure Function App for function python."
  exit 1
fi

sleep 1m

if ! func azure functionapp publish $function_name; then
  echo "Error: Failed to publish Azure Function App for function python."
  exit 1
fi

sleep 30s