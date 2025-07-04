resource "aws_s3_bucket" "website" {
  bucket = "image2xlsx-${data.aws_caller_identity.current.account_id}"
}

resource "aws_s3_bucket_policy" "website" {
  bucket = aws_s3_bucket.website.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid    = "CloudFront"
        Effect = "Allow"
        Principal = {
          Service = "cloudfront.amazonaws.com"
        }
        Action   = "s3:GetObject"
        Resource = "${aws_s3_bucket.website.arn}/*"
        Condition = {
          StringEquals = {
            "AWS:SourceArn" = aws_cloudfront_distribution.website.arn
          }
        }
      }
    ]
  })
}

resource "aws_s3_object" "website_files" {
  for_each = toset(local.website_files)

  bucket = aws_s3_bucket.website.id
  key    = each.value
  source = "${path.root}/../frontend/dist/${each.value}"
  etag   = filemd5("${path.root}/../frontend/dist/${each.value}")

  depends_on = [aws_s3_bucket_policy.website]
  content_type = (
    endswith(each.value, ".html") ? "text/html" : (
      endswith(each.value, ".css") ? "text/css" :
      endswith(each.value, ".js") ? "application/javascript" :
      endswith(each.value, ".png") ? "image/png" :
      (endswith(each.value, ".jpg") || endswith(each.value, ".jpeg")) ? "image/jpeg" :
      endswith(each.value, ".svg") ? "image/svg+xml" : "application/octet-stream"
    )
  )
}
