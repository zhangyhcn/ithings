
root@DESKTOP-KBAP4SU:/etc/rancher/k3s#
-rw-r--r-- 1 root root  338 Mar 18 13:36 registries.yaml
-rw-r--r-- 1 root root   39 Mar 18 13:36 v2



mirrors:
  "docker.io":
    endpoint:
      - "https://docker.m.daocloud.io"
      - "https://registry.cn-hangzhou.aliyuncs.com"
  "172.17.0.1:30500":
    endpoint:
      - "https://172.17.0.1:30500"
configs:
  "172.17.0.1:30500":
    tls:
      ca_file: "/etc/containerd/certs.d/172.17.0.1:30500/ca.crt"
      insecure_skip_verify: true
~
~
~
~
~
~
~