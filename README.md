# mandel-rust

## Deploy to GCP

### Create Cluster and apply ressouces

- gcloud components update
- gcloud container clusters create-auto mandel --region europe-west4
- kubectl get nodes
- kubectl apply -f deployment.yaml
- kubectl get pods
- kubectl apply -f backend_config.yaml
- kubectl get backendconfig
- kubectl apply -f service_np.yaml
- kubectl get services
- If only Rust
  - kubectl apply -f ingress_rust_only.yaml
- If Rust and Java
  - kubectl apply -f ingress.yaml
- kubectl get ingress

### Test 

Use that IP address which is shown with `kubectl get ingress` command.

- curl http://35.190.12.49/mandel-rust/mandel_text/256

### Delete ressources and cluster
- kubectl delete ingress mandel-ingress
- kubectl delete service mandel-rust-service
- kubectl delete backendconfig mandel-rust-backend-config
- kubectl delete deployment mandel-rust-deployment
- gcloud container clusters delete mandel --region europe-west4
