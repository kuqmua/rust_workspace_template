# Production overlay example

This overlay demonstrates immutable images, production guardrails, and secret references without
containing a credential. Replace the example origin and image digest, and create
`application-secrets` through the deployment secret provider. Do not commit the Secret object or
rendered secret values.

```bash
kubectl kustomize deploy/k8s/overlays/production-example
```

The base also references configuration for the notification service. A real environment must
supply that service's ConfigMap and Secret or explicitly remove the workload.
