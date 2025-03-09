variable "prefix" {
  description = "The prefix which should be used for all resources."
}

variable "environment" {
  description = "The type of environment to deploy"
}

variable "resource_group_name" {
  description = "The name of the resource group where the resources should be deployed to."
}

variable "system_topic_name" {
  description = "The primary blob connection string of the datalake storage account"
}

variable "subscription_id" {
  type = string
}

variable "function_app_name" {
  type = string
}

variable "storage_account_id" {
  type = string
}