rg=tfstate
stacc=devtfstate
container=tfstate

az account set --subscription SUBSCRIPTION
if ! az group create --name $rg --location westeurope; then
    echo "could not create azure resource group"
    exit 1
fi;

if ! az storage account create --resource-group $rg --name $stacc --sku Standard_LRS --encryption-services blob; then
    echo "could not create azure storage account"
    exit 1
fi;

if ! az storage container create --name $container --account-name $stacc; then
    echo "could not create azure storage container"
    exit 1
fi;

terraform init -backend-config="resource_group_name=$rg" \
    -backend-config="storage_account_name=$stacc" \
    -backend-config="container_name=$container" \
    -backend-config="key=sample.terraform.tfstate" \
    -backend-config="subscription_id=" \
    -backend-config="tenant_id=" \
    -backend-config="client_id=" \
    -backend-config="client_secret=" \
    -reconfigure