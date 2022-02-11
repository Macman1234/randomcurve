import mhcurve
import svgwrite

curve = mhcurve.mansfieldcurve(32,32)
curve.iterate(1000000)

dwg = svgwrite.Drawing('test5.svg', profile='tiny')
for i in range(len(curve.locs)-1):
    x1 = curve.locs[i][0]*8+10
    y1 = curve.locs[i][1]*8+10
    x2 = curve.locs[i+1][0]*8+10
    y2 = curve.locs[i+1][1]*8+10
    dwg.add(dwg.line((x1, y1), (x2, y2), stroke=svgwrite.rgb(10, 10, 16, '%')))
dwg.save()