#!/usr/bin/env python3
# Based on https://github.com/berdav/qemu-rpi-gpio/tree/master (GPLv3 licenced)

import re
import os
import time
import sys
import pexpect

SOCK_PATH="/tmp/qtest-gpio.sock"
GPIO_RANGE=[0x3f200000, 0x3f200fff]
IC_RANGE  =[0x3f00b200, 0x3f00b3ff]

GPIO_SET_OFFSET=0x1c
GPIO_RESET_OFFSET=0x28
GPIO_READ_OFFSET=0x34

class VGPIOManager(object):
    fd = None
    twenty_val = 0
    def __init__(self, spath=SOCK_PATH):
        self.load(spath)

    def load(self, spath=SOCK_PATH):
        if os.path.exists(SOCK_PATH):
            os.unlink(SOCK_PATH)

        self.fd = pexpect.spawn("socat - UNIX-LISTEN:{}".format(SOCK_PATH))

    def get_gpio_location(self, num):
        if num > 54 or num < 0:
            return 0
        return GPIO_RANGE[0] + int(num / 32)

    def _read(self):
        # Cancel echo
        self.fd.readline()
        return self.fd.readline()

    def writel(self, address, value):
        self._sendline('writel 0x{:x} 0x{:x}'.format(address, value))
        return self._read()

    def _sendline(self, s):
        return self.fd.sendline(s)

    def close(self):
        self.fd.close()

    def set(self, gpionum, value):
        m = self.get_gpio_location(gpionum)
        if value:
            m += GPIO_SET_OFFSET
        else:
            m += GPIO_RESET_OFFSET
        gpio = 1 << (gpionum % 32)
        return self.writel(m, gpio)

    def parse(self, s):
        s = s.split(' ')
        x = s[0]
        y = s[1]

        if self.twenty_val == 0:
            self.twenty_val = 1
        else:
            self.twenty_val = 0

        self.set(self.translate_x_to_pin(x), 1)
        self.set(self.translate_y_to_pin(y), 1)
        self.set(20, self.twenty_val)

        time.sleep(1)

        self.set(self.translate_x_to_pin(x), 0)
        self.set(self.translate_y_to_pin(y), 0)
        return "OK"


    def translate_x_to_pin(self, x):
        if x == "0":
            return 1 
        elif x == "1":
            return 7
        elif x == "2":
            return 8

    def translate_y_to_pin(self, x):
        if x == "0":
            return 10 
        elif x == "1":
            return 9
        elif x == "2":
            return 11

if __name__=="__main__":
    vgpio = VGPIOManager()
    while True:
        cmd = input('-> ')
        print(vgpio.parse(cmd))
    vgpio.close()

