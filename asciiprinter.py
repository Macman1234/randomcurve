import mhcurve
import curvetoascii
import time
 
if __name__ == "__main__":

    picturecount = 1

    itercount = 1000000
    xsize = 40
    ysize = 40

    for i in range(picturecount):
        curve = mhcurve.mansfieldcurve(xsize,ysize)
        starttime = time.perf_counter()
        curve.iterate(itercount)
        stoptime = time.perf_counter()
        print(f"curve {i}, {xsize} by {ysize}, did {itercount} iterations, took {stoptime-starttime} seconds:")
        print(curvetoascii.curvetoascii(curve))