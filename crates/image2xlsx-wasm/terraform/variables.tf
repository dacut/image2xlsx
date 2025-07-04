variable "aws_region" {
  description = "The AWS region to deploy resources in"
  type        = string
  default     = "us-west-2"
}

variable "domain_name" {
  description = "The domain name to use for the website"
  type        = string
  default     = "image2xlsx.kanga.org"
}

variable "route53_zone_id" {
  description = "The Route 53 zone ID for the domain"
  type        = string
  default     = "Z27TOGF84F4SCW" # Replace with your actual Route 53 zone ID
}
