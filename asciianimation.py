import mhcurve
import curvetoascii
import cursorhide
import os
import atexit

itercount = 1000000
itersize = 1
term_size = os.get_terminal_size()
xsize = term_size.columns
ysize = term_size.lines
curve = mhcurve.mansfieldcurve(xsize,ysize)
cursorhide.hide_cursor()
atexit.register(cursorhide.show_cursor)
for i in range(itercount):
    curve.iterate(itersize)
    print(curvetoascii.curvetoascii(curve),end="")
    print("\r",end="")
    print("\033[A"*(ysize+1),end="")
    #time.sleep(0.1)