import mhcurve
import matplotlib.pyplot as plt

picturecount = 10

itercount = 100000
xsize = 32
ysize = 16

for i in range(picturecount):
    curve = mhcurve.mansfieldcurve(xsize,ysize)
    curve.iterate(itercount)

    x = [loc[0] for loc in curve.locs]
    y = [loc[1] for loc in curve.locs]

    fig = plt.figure(figsize=(4, 2), dpi=1200)
    plt.plot(x,y,linewidth=3,color="#000000")
    plt.axis('off')

    plt.savefig(f"out{i}.png", bbox_inches=0)
    plt.close(fig)