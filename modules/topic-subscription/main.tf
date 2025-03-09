resource "azurerm_eventgrid_system_topic_event_subscription" "event_subscription_input" {
  name                  = "${var.prefix}${var.environment}-input-subscription"
  system_topic          = var.system_topic_name
  resource_group_name   = var.resource_group_name
  event_delivery_schema = "EventGridSchema"

  azure_function_endpoint {
    function_id                       = "/subscriptions/${var.subscription_id}/resourceGroups/${var.resource_group_name}/providers/Microsoft.Web/sites/${var.function_app_name}/functions/eventGridTrigger"
    max_events_per_batch              = 1
    preferred_batch_size_in_kilobytes = 64
  }

  included_event_types = [
    "Microsoft.Storage.BlobCreated", # https://learn.microsoft.com/en-us/azure/event-grid/event-schema-blob-storage?tabs=cloud-event-schema.
    "Microsoft.Storage.BlobDeleted"
  ]
}