import math
from random import randrange

class mansfieldcurve:
    def __init__(self,x, y):
        self.asize = [x,y] # because size was taken? i forget.
        self.locs = []
        
        # generate path snaking back and forth across
        # this should probably be a hilbert curve or something with more
        # uniform density but with enough iterations given the size this is fine
        # if you ever want to use this for something more serious than
        # simple graphics, please change this.
        # i'm not joking if this is used for science i'll be very pissed
        # if someone leaves it at a snaking path without investigating
        # whether that causes distribution problems
        
        for i in range(self.asize[0]*self.asize[1]):
            currenty = math.floor(i / x)
            if not (currenty % 2):
                currentx = i % x
            else:
                currentx = x - (i % x) - 1
            self.locs.append([currentx,currenty])
    
    def iterate(self,num):
        # for each iteration, do one monte carlo swap
        for n in range(num):
            beginorend = randrange(2) # choose whether to start for beginning point or end point
            if not beginorend:
                p = 0 # starter index
            else:
                p = len(self.locs)-1 # ending index
            pos = self.locs[p]
            neighbors = [[pos[0]-1,pos[1]],[pos[0]+1,pos[1]],[pos[0],pos[1]-1],[pos[0],pos[1]+1]] # find position of neighbors
            looking = True
            while looking: # bruteforce loop through to look for indeces of neighbors. This sucks!!!
                chooseneighbor = neighbors[randrange(4)]
                try:
                    p2 = self.locs.index(chooseneighbor)
                    if abs(p2 - p) != 1: # if it's not the neighbor that's already connected, stop looking.
                        looking = False
                except ValueError:
                    looking = True
            pos2 = chooseneighbor
            
            # see the cited mansfield paper for info on this operation. involves reversing the list between
            # the two points
            if not beginorend: # swap chosen point around beginning
                section = self.locs[0:p2:1]
                section.reverse()
                self.locs = section+self.locs[p2:]
            else: # swap chosen point around end
                section = self.locs[p2+1:]
                section.reverse()
                self.locs = self.locs[0:p2+1] + section
            
                
                
if __name__ == "__main__": # debugging demo - print starter 3x3 curve and curve after 10 iter
    curve = mansfieldcurve(3,3)
    print(curve.locs)
    curve.iterate(10)
    print(curve.locs)
    