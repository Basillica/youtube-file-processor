#!/bin/sh
set -e
rg=tfstate
stacc=devtfstate
container=tfstate

terraform fmt -recursive

terraform init -backend-config="resource_group_name=$rg" \
    -backend-config="storage_account_name=$stacc" \
    -backend-config="container_name=$container" \
    -backend-config="key=sample.terraform.tfstate" \
    -backend-config="subscription_id=" \
    -backend-config="tenant_id=" \
    -backend-config="client_id=" \
    -backend-config="client_secret=" \
    -reconfigure

terraform plan -var-file=variables.dev.tfvars
terraform apply -var-file=variables.dev.tfvars -auto-approve