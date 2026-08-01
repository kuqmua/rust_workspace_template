# Production overlay example

This overlay demonstrates immutable images, production guardrails, and secret references without
containing a credential. Replace the example origin and image digest, and create
`application-secrets` through the deployment secret provider. Do not commit the Secret object or
rendered secret values.

```bash
kubectl kustomize deploy/k8s/overlays/production-example
```

Before applying a customized render, run:

```bash
kubectl kustomize deploy/k8s/overlays/production-example > production.yaml
deploy/validate-production-manifest.sh production.yaml
```

The validator fails closed while example domains, replacement image values, non-digest service
images, loopback-only trusted proxy ranges, or required production security resources remain in
the render. Set `rust-workspace-template/config-revision` to a new deployment-owned value whenever
either generated ConfigMap changes; the pod-template annotation then triggers the required rollout
despite the stable ConfigMap names shared with migration Jobs.

The overlay supplies non-secret configuration for the notification service. A real environment
must supply that service's Secret or explicitly remove the workload.

The example intentionally exposes only the application workload to the ingress namespace. The
notification service, including its `/metrics` endpoint, accepts traffic from the application pods
only. Supply `database-url` and `admin-jwt-secret` in `application-secrets`, and
`notification-database-url` in `notification-service-secrets` through the deployment secret
provider.

For Prometheus Operator installations, render `deploy/k8s/observability` separately after replacing
its example runbook URL. The `monitoring` namespace must retain the standard
`kubernetes.io/metadata.name=monitoring` label used by the base metrics NetworkPolicy.
