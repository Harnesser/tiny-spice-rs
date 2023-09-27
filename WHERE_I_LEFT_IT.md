Fixed parameter lookup, by the looks of it...

This still doesn't work:

```spice
Xsystem1 IN_p gnd vp1 vn1 system cval=1uF
Xsystem2 IN_p gnd vp2 vn2 system ; FIXME cval=10uF
Xsystem3 IN_p gnd vp3 vn3 system cval=100uF
```
