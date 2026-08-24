# 1. Turn off WSL's automated, broken DNS file generator
sudo bash -c 'cat <<EOF > /etc/wsl.conf
[network]
generateResolvConf = false
EOF'

# 2. Delete the broken symbolic link file
sudo rm -f /etc/resolv.conf

# 3. Create a static, unmanaged file pointing straight to Cloudflare and Google DNS
sudo bash -c 'cat <<EOF > /etc/resolv.conf
nameserver 1.1.1.1
nameserver 8.8.8.8
EOF'
