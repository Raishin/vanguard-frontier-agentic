# Official sources

Use this reference only when you need source grounding for Alibaba Cloud load balancer service behavior or the detailed source list.

## Alibaba Cloud documentation

Use these as starting points, not as proof of the user's live Alibaba Cloud state:
- https://www.alibabacloud.com/help/en/slb/classic-load-balancer/product-overview/what-is-clb
- https://www.alibabacloud.com/help/en/slb/application-load-balancer/product-overview/what-is-alb
- https://www.alibabacloud.com/help/en/slb/application-load-balancer/user-guide/create-an-https-listener
- https://www.alibabacloud.com/help/en/slb/network-load-balancer/product-overview/what-is-nlb
- https://www.alibabacloud.com/help/en/global-accelerator/latest/what-is-global-accelerator
- https://www.alibabacloud.com/help/en/waf/latest/what-is-waf
- https://www.alibabacloud.com/help/en/ssl-certificate/latest/what-is-ssl-certificates-service

## Grounding rule

Official documentation explains Alibaba Cloud LB service behavior and feature availability. It does not prove the user's current listener configuration, health check status, backend health, or WAF rule state. Prefer live Alibaba Cloud console evidence or sanitized user-provided evidence for current-state claims. NLB health check limitations (TCP only, no HTTP) are product constraints documented officially — do not assume HTTP health check support for NLB.
