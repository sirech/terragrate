provider "docker" {
  version = "~> 2.7"
}

resource "docker_image" "image" {
  name = "nginx:alpine"
}

resource "docker_network" "network" {
  name = "my_network"
}

resource "docker_container" "container" {
  image = docker_image.image.name
  name  = "container"

  command    = ["nginx", "-g", "daemon off;"]
  entrypoint = []
  env        = []

  privileged        = false
  publish_all_ports = false

  max_retry_count = 0
  memory          = 0
  memory_swap     = 0
  shm_size        = 64

  networks_advanced {
    name = docker_network.network.name
  }
}
