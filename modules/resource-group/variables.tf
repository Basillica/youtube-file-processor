variable "resource_group_name" {
  type        = string
  description = "The name of an existing resource group."
}

variable "environment" {
  description = "The name of environment to deploy"
}

variable "location" {
  description = "The Azure location where the RG should be created."
}
