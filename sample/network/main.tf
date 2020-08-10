resource "docker_network" "network" {
  name     = var.name
  internal = var.internal
}
