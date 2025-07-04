terraform {
  required_version = ">= 1.10"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 6.0"
    }
  }
}

provider "aws" {
  region = var.aws_region

  default_tags {
    tags = {
      Project   = "image2xlsx"
      Terraform = "github.com/dacut/image2xlsx/crates/image2xlsx-wasm/terraform"
    }
  }
}

data "aws_caller_identity" "current" {}
data "aws_partition" "current" {}

locals {
  website_files = fileset("${path.root}/../frontend/dist", "**/*")
}
