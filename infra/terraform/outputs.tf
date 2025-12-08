output "cluster_name" {
  description = "GKE Cluster Name"
  value       = google_container_cluster.primary.name
}

output "cluster_endpoint" {
  description = "GKE Cluster Endpoint"
  value       = google_container_cluster.primary.endpoint
  sensitive   = true
}

output "cluster_ca_certificate" {
  description = "GKE Cluster CA Certificate"
  value       = google_container_cluster.primary.master_auth[0].cluster_ca_certificate
  sensitive   = true
}

output "ingress_ip" {
  description = "Static IP for Ingress"
  value       = google_compute_global_address.ingress_ip.address
}

output "artifact_registry_url" {
  description = "Artifact Registry URL"
  value       = "${var.region}-docker.pkg.dev/${var.project_id}/${google_artifact_registry_repository.docker.repository_id}"
}

output "cicd_service_account_email" {
  description = "CI/CD Service Account Email"
  value       = google_service_account.cicd.email
}
