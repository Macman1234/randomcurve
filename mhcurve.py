import math
from random import randrange

class mansfieldcurve:
    def __init__(self,x, y):
        self.asize = [x,y]
        self.locs = []
        
        for i in range(self.asize[0]*self.asize[1]):
            currenty = math.floor(i / x)
            if not (currenty % 2):
                currentx = i % x
            else:
                currentx = x - (i % x) - 1
            self.locs.append([currentx,currenty])
    
    def iterate(self,num):
        for n in range(num):
            beginorend = randrange(2)
            if not beginorend:
                p = 0
                #print("starting from beginning")
            else:
                p = len(self.locs)-1
                #print("starting from end")
            pos = self.locs[p]
            neighbors = [[pos[0]-1,pos[1]],[pos[0]+1,pos[1]],[pos[0],pos[1]-1],[pos[0],pos[1]+1]]
            looking = True
            while looking:
                chooseneighbor = neighbors[randrange(4)]
                #print(chooseneighbor)
                try:
                    p2 = self.locs.index(chooseneighbor)
                    if abs(p2 - p) != 1: #if it's already connected, stop.
                        looking = False
                except ValueError:
                    looking = True
            pos2 = chooseneighbor
            #print(pos)
            #print(pos2)
            #print(p)
            #print(p2)
            if not beginorend:
                section = self.locs[0:p2:1]
                #print(section)
                section.reverse()
                #print(section)
                self.locs = section+self.locs[p2:]
            else:
                section = self.locs[p2+1:]
                #print(section)
                section.reverse()
                #print(section)
                self.locs = self.locs[0:p2+1] + section
            
                
                
if __name__ == "__main__":
    curve = mansfieldcurve(3,3)
    print(curve.locs)
    curve.iterate(10)
    print(curve.locs)
    