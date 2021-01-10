# atlc-gen
A small utility to generate bitmaps for [atlc](http://atlc.sourceforge.net/).

## Options

    -r, --resolution <resolution>
      Pixel resolution. Defines the units of all other values, e.g a value of 5
      (micrometers, the default) results in a precision of 5 µm if all other
      values are given in µm.
    -C, --core-thickness <core-thickness>
      Thichness of the PCB core between top and bottom layer.
    -c, --cu-thickness <cu-thickness>
      Thichness of the copper layers.
    -f, --out-filename <filename>
      Name of the generated bitmap file.
    -S, --inner-space <inner-space>
      Space between two differential PCB traces.
    -s, --outer-space <outer-space>
      Space between the PCB traces and top ground plane.
    -x <res_x>
      Width of the simulation area.
    -y <res_y>
      Height of the simulation area.
    -m, --sm-thickness <sm-thickness>
      Soldermask thickness around top layer.
    -t, --trace-width <trace-width>
      Width of the PCB traces.
    -v, --via-fence-dist <via-fence-dist>
      Via fence distance from top ground plane.
    -V, --via-fence-thickness <via-fence-thickness>
      Via thickness.


