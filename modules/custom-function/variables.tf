variable "location" {
  type        = string
  description = "Azure region to deploy to"
}

variable "function_name" {
  type = string
}

variable "prefix" {
  description = "The prefix which should be used for all resources."
  type        = string
}

variable "environment" {
  description = "The type of environment to deploy"
  type        = string
}

variable "resource_group_name" {
  description = "The name of the resource group where the resources should be deployed to."
  type        = string
}

variable "app_settings" {
  type = map(string)
  default = {
    environment = "dev"
  }
  description = "Map of App Settings."
}

variable "function_source_code_path" {
  type = string
}