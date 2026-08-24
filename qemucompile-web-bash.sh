# Update system dependencies
sudo apt update && sudo apt install -y git python3 python3-pip python3-venv build-essential pkg-config

# Clone and activate the Emscripten SDK
git clone https://github.com ~/emsdk
cd ~/emsdk
./emsdk install latest
./emsdk activate latest
source ./emsdk_env.sh

# Navigate to your repository root on your Windows drive (e.g., C:\Users\YourName\Documents\Repo)
cd /mnt/c/programming/RS/HPVMx/qemu-web-src

# 1. Reset toolchain flags to prevent host pollution
unset EMCC_CFLAGS EMXX_CFLAGS

# 2. Package your existing host firmware and VHD files straight into the compiler string
export EMCC_CFLAGS="-s WASM_BIGINT=1 -s ALLOW_MEMORY_GROWTH=1 -s ASYNCIFY=1 --preload-file ../code.fd@/code.fd --preload-file ../vars.fd@/vars.fd"
export EMXX_CFLAGS="-s WASM_BIGINT=1 -s ALLOW_MEMORY_GROWTH=1 -s ASYNCIFY=1 --preload-file ../code.fd@/code.fd --preload-file ../vars.fd@/vars.fd"

# 3. Spin up your Python environment and run configure natively
python3 -m venv ../pyvenv
../pyvenv/bin/pip install meson==1.2.3

emconfigure ./configure \
  --target-list=x86_64-softmmu \
  --cpu=wasm32 \
  --cross-prefix= \
  --static \
  --disable-tools \
  --disable-cocoa \
  --disable-sdl \
  --disable-gtk \
  --disable-vnc \
  --disable-kvm \
  --disable-xen \
  --disable-guest-agent \
  --disable-pie \
  --disable-modules \
  --disable-docs \
  --enable-download

# 4. Use the specific virtual environment path to trigger meson setup with forcefallback
../pyvenv/bin/meson setup build \
  --cross-file emscripten-cross.txt \
  --wrap-mode=forcefallback \
  --reconfigure \
  -Ddefault_library=static \
  -Dtools=disabled \
  -Dcocoa=disabled \
  -Dsdl=disabled \
  -Dgtk=disabled \
  -Dvnc=disabled \
  -Dkvm=disabled \
  -Dxen=disabled \
  -Dguest_agent=disabled \
  -Ddocs=disabled \
  -Db_pie=false

# 5. Compile using all of your local CPU cores
../pyvenv/bin/meson compile -C build -j$(nproc)
