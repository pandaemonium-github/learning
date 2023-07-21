## change sound output from command line

List sound outputs:
``` sh
pacmd list-sinks | grep -e 'name:' -e 'index:'
```

```
    index: 0
        name: <alsa_output.pci-0000_03_00.1.hdmi-stereo-extra2>
  * index: 1
        name: <alsa_output.usb-Logitech_G533_Gaming_Headset-00.iec958-stereo>
    index: 2
        name: <alsa_output.pci-0000_00_1f.3-platform-skl_hda_dsp_generic.HiFi__hw_sofhdadsp_5__sink>
    index: 3
        name: <alsa_output.pci-0000_00_1f.3-platform-skl_hda_dsp_generic.HiFi__hw_sofhdadsp_4__sink>
    index: 4
        name: <alsa_output.pci-0000_00_1f.3-platform-skl_hda_dsp_generic.HiFi__hw_sofhdadsp_3__sink>
    index: 5
        name: <alsa_output.pci-0000_00_1f.3-platform-skl_hda_dsp_generic.HiFi__hw_sofhdadsp__sink>
```
Change output to output 0:
```sh
pacmd set-default-sink alsa_output.pci-0000_03_00.1.hdmi-stereo-extra2
```

Note: [ref](https://askubuntu.com/questions/14077/how-can-i-change-the-default-audio-device-from-command-line)
```
Note: Changing the output sink through the command line interface can only take effect if stream target device reading is disabled. This can be done by editing the corresponding line in /etc/pulse/default.pa to:

load-module module-stream-restore restore_device=false

Restart PulseAudio for changes to take effect:

pulseaudio -k
```
