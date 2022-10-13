# Emulating Raspberry Pi 3b

## Step 1: Run Raspbian
- followed tutorial from: <https://blog.agchapman.com/using-qemu-to-emulate-a-raspberry-pi/>
- download the nessicary images:
```bash
wget https://github.com/dhruvvyas90/qemu-rpi-kernel/raw/master/kernel-qemu-4.4.34-jessie
wget https://downloads.raspberrypi.org/raspbian_lite/images/raspbian_lite-2017-08-17/2017-08-16-raspbian-stretch-lite.zip
unzip 2017-08-16-raspbian-stretch-lite.zip
```
- reformat the disk image:
```bash
qemu-img convert -f raw -O qcow2 2017-08-16-raspbian-stretch-lite.img raspbian-stretch-lite.qcow2
qemu-img resize raspbian-stretch-lite.qcow +6G
```
- start qemu:
```bash
sudo qemu-system-arm \
-kernel ./kernel-qemu-4.4.34-jessie \
-append "root=/dev/sda2 panic=1 rootfstype=ext4 rw" \
-hda raspbian-stretch-lite.qcow \
-cpu arm1176 -m 256 \
-M versatilepb \
-no-reboot \
-serial stdio \
-net nic -net user \
-net tap,ifname=vnet0,script=no,downscript=no
```
- running qemu suceeded:
![qemu with raspbian](./images/rasbian_qemu_custom_kernel.png)


## Current Status

- Attepting to run iso in qemu results in black screen:
```bash
qemu-system-aarch64 -machine type=raspi3b -m 1024 -cpu cortex-a53 -hda ubuntu.img
```
- According to some, it is impossible to run the default kernel in qemu


