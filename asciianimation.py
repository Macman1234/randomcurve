import mhcurve
import curvetoascii
import cursorhide
import os
import atexit

if __name__ == "__main__":
    itersize = 1
    resizechecker = 20 # to avoid calling os.get_terminal_size on every iteration
    cursorhide.hide_cursor()
    atexit.register(cursorhide.show_cursor)
    while True:
        term_size = os.get_terminal_size()
        xsize = term_size.columns
        ysize = term_size.lines
        curve = mhcurve.mansfieldcurve(xsize,ysize)
        resize = False
        resizecount = 0
        while not resize:
            resizecount += 1
            if resizecount % resizechecker == 0:
                resize = not (term_size == os.get_terminal_size())
            curve.iterate(itersize)
            print(curvetoascii.curvetoascii(curve),end="")
            print("\r",end="")
            print("\033[A"*(ysize+1),end="")
            #time.sleep(0.1)