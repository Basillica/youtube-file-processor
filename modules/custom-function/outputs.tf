output "function_name" {
  value = azurerm_windows_function_app.custom_handler.name
}

output "function_app_name" {
  value       = azurerm_windows_function_app.custom_handler.name
  description = "Deployed function app name"
}

output "function_id" {
  value = azurerm_windows_function_app.custom_handler.id
}

output "system_topic_name" {
  value = azurerm_eventgrid_system_topic.event_topic.name
}

output "storage_acocunt_id" {
  value = azurerm_storage_account.blob_store.id
}