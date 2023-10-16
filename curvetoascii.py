# takes in a list of positions on a grid as a curve
# and composes an ascii representation for that grid.


def getcurvechar(curve,idx):
    bold = False
    arc = True
    if idx == 0:
        neighbordiffs = [
            [curve.locs[idx][0]-curve.locs[idx+1][0],curve.locs[idx][1]-curve.locs[idx+1][1]]
        ]
    elif idx == len(curve.locs)-1:
        neighbordiffs = [
            [curve.locs[idx][0]-curve.locs[idx-1][0],curve.locs[idx][1]-curve.locs[idx-1][1]]
        ]
    else:
        neighbordiffs = [
            [curve.locs[idx][0]-curve.locs[idx-1][0],curve.locs[idx][1]-curve.locs[idx-1][1]],
            [curve.locs[idx][0]-curve.locs[idx+1][0],curve.locs[idx][1]-curve.locs[idx+1][1]]
        ]
    #print(neighbordiffs)
    above = [0,1]
    below = [0,-1]
    right = [-1,0]
    left = [1,0]
    if below in neighbordiffs and right in neighbordiffs:
        if bold: return("┏")
        elif arc: return("╭")
        else: return("┌")
    if below in neighbordiffs and left in neighbordiffs:
        if bold: return("┓")
        elif arc: return("╮")
        else: return("┐")
    if above in neighbordiffs and right in neighbordiffs:
        if bold: return("┗")
        elif arc: return("╰")
        else: return("└")
    if above in neighbordiffs and left in neighbordiffs:
        if bold: return("┛")
        elif arc: return("╯")
        else: return("┘")
    if above in neighbordiffs or below in neighbordiffs:
        if bold: return("┃")
        else: return("│")
    if right in neighbordiffs or left in neighbordiffs:
        if bold: return("━")
        else: return("─")

def curvetoascii(curve):
    out = ""
    for y in range(curve.dim[1]): # scan y-axis
        for x in range(curve.dim[0]): # scan x-axis
            #print(curve.locs)
            try:
                idx = curve.locs.index([x,y])
                out += getcurvechar(curve,idx)
            except ValueError:
                out += " "
        if y < curve.dim[1]-1: out += "\n"
    return(out)