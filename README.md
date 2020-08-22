# toby-pictures

![Build](https://github.com/gorup/toby-pictures/workflows/Build/badge.svg)

Example repo of a website that uses [`ebbflow`](https://ebbflow.io) as the hosting platform.

Using kubernetes (see `deployment.yaml`) and I can host my little example website without dealing with ingress, load balancers, IPs, or anything at all.

You can get this site running in your own kubernetes cluster by following these steps:

1. Follow [this guide from StackOverflow](https://stackoverflow.com/questions/61912589/how-can-i-use-github-packages-docker-registry-in-kubernetes-dockerconfigjson) to allow your Kubernetes cluster pull GitHub images
1. The only other set up is to provide the `EBB_KEY` environment variable, hopefully through a kubernetes secret. I created my secret like this:
    ```
    kubectl create secret generic ebbkey --from-literal=ebbkey=ebb_hst_RMsnwxXMBrdiFRHgDJQHMGJaasdf1234
    ```
1. Get the `deployment.yaml` ready: **Change** the `--dns` value to a domain name you have created an endpoint for on [ebbflow.io](https://ebbflow.io).
1. Apply this yaml via something like `kubectl apply -f deployment.yaml`.
1. Done!
