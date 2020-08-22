# toby-pictures

![Build](https://github.com/gorup/toby-pictures/workflows/Build/badge.svg)

Example repo of a website that uses [`ebbflow`](https://ebbflow.io) as the hosting platform.

Using kubernetes (see `deployment.yaml`) and I can host my little example website without dealing with ingress, load balancers, IPs, or anything at all.

You can try it yourself, follow [this guide from StackOverflow](https://stackoverflow.com/questions/61912589/how-can-i-use-github-packages-docker-registry-in-kubernetes-dockerconfigjson) to allow your Kubernetes cluster pull GitHub images, and then you should be able to just copy my deployment.yaml and apply it.

The only other set up is to provide the `EBB_KEY` environment variable, hopefully through a kubernetes secret. I created my secret like this:

```
kubectl create secret generic ebbkey --from-literal=ebbkey=ebb_hst_RMsnwxXMBrdiFRHgDJQHMGJaasdf1234
```
