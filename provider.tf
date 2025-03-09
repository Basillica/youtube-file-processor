terraform {
  backend "azurerm" {
  }

  required_providers {
    rm = {
      source  = "hashicorp/azurerm"
      version = "~>3.8"
    }
  }
}

provider "rm" {
  features {}
  subscription_id = var.subscription_id
  client_id       = var.client_id
  client_secret   = var.client_secret
  tenant_id       = var.tenant_id
}