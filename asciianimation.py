import mhcurve
import curvetoascii
import time

itercount = 1000000
itersize = 1
xsize = 60
ysize = 10
curve = mhcurve.mansfieldcurve(xsize,ysize)
for i in range(itercount):
    curve.iterate(itersize)
    print(curvetoascii.curvetoascii(curve))
    print("\r",end="")
    print("\033[A"*(ysize+1),end="")
    #time.sleep(0.1)