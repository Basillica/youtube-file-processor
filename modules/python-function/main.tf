resource "azurerm_storage_account" "storage_account" {
  name                     = "${var.function_name}stacc"
  resource_group_name      = var.resource_group_name
  location                 = var.location
  account_tier             = "Standard"
  account_replication_type = "LRS"
}

resource "azurerm_storage_container" "data_store" {
  name                  = "${var.function_name}-container"
  storage_account_name  = azurerm_storage_account.storage_account.name
  container_access_type = "private"
}


resource "azurerm_application_insights" "app_insights" {
  name                = "${var.function_name}insights"
  resource_group_name = var.resource_group_name
  location            = var.location
  application_type    = "web"
}

resource "azurerm_service_plan" "app_service_plan" {
  name                = "${var.function_name}svcplan"
  resource_group_name = var.resource_group_name
  location            = var.location
  os_type             = "Linux"
  sku_name            = "Y1"
}

####################
resource "azurerm_storage_account" "blob_store" {
  name                     = "${var.function_name}store"
  resource_group_name      = var.resource_group_name
  location                 = var.location
  account_tier             = "Standard"
  account_replication_type = "LRS"
}

resource "azurerm_storage_container" "storage_queue" {
  name                  = "${var.function_name}-blob-container"
  storage_account_name  = azurerm_storage_account.blob_store.name
  container_access_type = "private"
}

resource "azurerm_eventgrid_system_topic" "event_topic" {
  name                   = "${var.function_name}-topic"
  resource_group_name    = var.resource_group_name
  location               = var.location
  source_arm_resource_id = azurerm_storage_account.blob_store.id
  topic_type             = "Microsoft.Storage.StorageAccounts" # az eventgrid topic-type list --output json | grep -w id

  tags = {
    stage = "dev"
  }
}
##################################

resource "azurerm_linux_function_app" "custom_handler" {
  name                = var.function_name
  resource_group_name = var.resource_group_name
  location            = var.location

  storage_account_name       = azurerm_storage_account.storage_account.name
  storage_account_access_key = azurerm_storage_account.storage_account.primary_access_key
  service_plan_id            = azurerm_service_plan.app_service_plan.id

  https_only = true
  app_settings = merge(
    {
      "APPINSIGHTS_INSTRUMENTATIONKEY" = "${azurerm_application_insights.app_insights.instrumentation_key}",
      StorageAccountConnectionString   = "${azurerm_storage_account.blob_store.primary_access_key}"
    },
    var.app_settings
  )

  functions_extension_version = "~4"
  zip_deploy_file             = data.archive_file.python_function_package.output_path

  site_config {
    application_insights_connection_string = azurerm_application_insights.app_insights.connection_string
    application_insights_key               = azurerm_application_insights.app_insights.instrumentation_key
    ftps_state                             = "Disabled"
    http2_enabled                          = true

    application_stack {
      python_version = "3.9"
    }

    app_service_logs {
      retention_period_days = 30
    }

    cors {
      allowed_origins     = ["*"]
      support_credentials = false
    }

    minimum_tls_version = "1.2"
  }
}

data "archive_file" "python_function_package" {
  type        = "zip"
  source_dir  = var.function_source_code
  output_path = "${var.function_name}.zip"
  excludes = [
    "**/*.pyc",
    "**/__pycache__",
    "**/*.pem",
    "venv", "env", ".venv", ".env"
  ]
}