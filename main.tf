# module "custom_funcz" {
#   source = "./modules/custom-function"

#   resource_group_name = module.rg.resource_group_name
#   location            = "westeurope"
#   app_settings = {
#     STAGE = "dev"
#   }
#   environment               = "dev"
#   prefix                    = "sample"
#   function_source_code_path = "./functions/function-edge-rust"
#   function_name             = "samplefunctonie"

#   depends_on = [module.rg]
# }

# module "system_topic_subscription" {
#   source = "./modules/topic-subscription"

#   resource_group_name = module.rg.resource_group_name
#   environment         = "dev"
#   prefix              = "sample"
#   system_topic_name   = module.custom_funcz.system_topic_name
#   function_app_name   = module.custom_funcz.function_app_name
#   subscription_id     = var.subscription_id
#   storage_account_id  = module.custom_funcz.storage_acocunt_id

#   depends_on = [module.custom_funcz, module.rg]
# }

module "funcz_python" {
  source = "./modules/python-function"

  resource_group_name = module.rg.resource_group_name
  location            = "westeurope"
  app_settings = {
    STAGE = "dev"
  }
  function_source_code = "./functions/cron-function"
  function_name        = "samplefunctonie"

  depends_on = [module.rg]
}

module "system_topic_sub_python" {
  source = "./modules/topic-subscription"

  resource_group_name = module.rg.resource_group_name
  environment         = "dev"
  prefix              = "sample"
  system_topic_name   = module.funcz_python.system_topic_name
  function_app_name   = module.funcz_python.function_app_name
  subscription_id     = var.subscription_id
  storage_account_id  = module.funcz_python.storage_acocunt_id

  depends_on = [module.funcz_python, module.rg]
}

module "rg" {
  source = "./modules/resource-group"

  environment         = local.environment
  location            = local.region
  resource_group_name = local.resource_group_name
}