import mhcurve
import matplotlib.pyplot as plt

itercount = 100000
xsize = 24
ysize = 24

curve = mhcurve.mansfieldcurve(xsize,ysize)
curve.iterate(itercount)

x = [loc[0] for loc in curve.locs]
y = [loc[1] for loc in curve.locs]

plt.figure(figsize=(4, 4), dpi=1200)
plt.plot(x,y,linewidth=3,color="#414040")
plt.axis('off')

plt.savefig("test3.png", bbox_inches=0)