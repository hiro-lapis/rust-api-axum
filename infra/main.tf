variable "aws_region" {
  type        = string
  default     = "ap-northeast-1"
}
variable "aws_profile" {
  type        = string
  default     = "april_hiro"
}

terraform {
  required_version = "~>1"

  required_providers {
    aws = {
        source = "hashicorp/aws"
        version = "~>5"
    }
  }

    backend "s3" {
        bucket = "rust-api-terra-form"
        region = "ap-northeast-1"
        profile = "april_hiro"
        key = "rust-api.tfstate"
        encrypt = true
    }
}

provider "aws" {
    region = var.aws_region
    profile = var.aws_profile
}

module "network" {
    source = "./network"
}

module "middleware" {
  source                                   = "./middleware"
  book_app_db_subnet_group_name            = module.network.book_app_db_subnet_group_name
  book_app_db_security_group_id            = module.network.book_app_db_security_group_id
  book_app_redis_security_group_id         = module.network.book_app_redis_security_group_id
  book_app_vpc_connector_security_group_id = module.network.book_app_vpc_connector_security_group_id
  book_app_codebuild_security_group_id     = module.network.book_app_codebuild_security_group_id
  book_app_redis_subnet_group_name         = module.network.book_app_redis_subnet_group_name
}

module "secrets" {
  source = "./secrets"
}

module "app" {
  source                        = "./app"
  book_app_vpc_id               = module.network.book_app_vpc_id
  book_app_db_subnet_ids        = module.network.book_app_db_subnet_ids
  book_app_vpc_connector_sg_id  = module.network.book_app_vpc_connector_security_group_id
  book_app_secrets_manager_arn  = module.secrets.book_app_secrets_manager_arn
  apprunner_instance_role_arn   = module.iam.apprunner_instance_role_arn
  apprunner_ecr_access_role_arn = module.iam.apprunner_ecr_access_role_arn
}

module "iam" {
  source                       = "./iam"
  book_app_db_subnet_arns      = module.network.book_app_db_subnet_arns
  book_app_private_subnet_arns = module.network.book_app_private_subnet_arns
}
