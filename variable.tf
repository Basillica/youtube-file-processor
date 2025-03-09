variable "subscription_id" {
  type = string
}

variable "client_id" {
  type = string
}

variable "client_secret" {
  type = string
}

variable "tenant_id" {
  type = string
}

variable "resource_group_name" {
  type = string
}

variable "location" {
  description = "The Azure Region in which all resources should be created (lower case, without spaces)."
}

variable "prefix" {
  description = "The prefix which should be used for all resources."
  # short name, because storage account must be between 3 and 24 characters long (+environment+8)
}

variable "environment" {
  type = string
}