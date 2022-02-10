import ndmansfield

curve = ndmansfield.ndm(16,16)
curve.iterate(10000)

import svgwrite

dwg = svgwrite.Drawing('test4.svg', profile='tiny')
for i in range(len(curve.locs)-1):
    x1 = curve.locs[i][0]*10+10
    y1 = curve.locs[i][1]*10+10
    x2 = curve.locs[i+1][0]*10+10
    y2 = curve.locs[i+1][1]*10+10
    dwg.add(dwg.line((x1, y1), (x2, y2), stroke=svgwrite.rgb(10, 10, 16, '%')))
dwg.save()