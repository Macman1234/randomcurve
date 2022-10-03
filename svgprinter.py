import mhcurve
import svgwrite

# these should eventually be command line arguments
filename = "test6"
itercount = 100000
xsize = 64
ysize = 32

curve = mhcurve.mansfieldcurve(xsize,ysize)
curve.iterate(itercount)

# draw it using svgwrite
dwg = svgwrite.Drawing(filename+".svg", profile='tiny')
for i in range(len(curve.locs)-1):
    x1 = curve.locs[i][0]*8+10
    y1 = curve.locs[i][1]*8+10
    x2 = curve.locs[i+1][0]*8+10
    y2 = curve.locs[i+1][1]*8+10
    dwg.add(dwg.line((x1, y1), (x2, y2), stroke=svgwrite.rgb(10, 10, 16, '%')))
dwg.save()