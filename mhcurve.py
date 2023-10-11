import math
from multiprocessing import Pool
from random import randrange

class mansfieldcurve:
    def __init__(self,x, y):
        self.dim = [x,y]
        self.locs = []
        
        # generate path snaking back and forth across
        # this should probably be a hilbert curve or something with more
        # uniform density but with enough iterations given the size this is fine
        # if you ever want to use this for something more serious than
        # simple graphics, please change this.
        
        # if this is used for science i'll be very mad
        # if someone leaves it at a snaking path without investigating
        # whether that causes distribution problems
        
        for i in range(self.dim[0]*self.dim[1]):
            currenty = math.floor(i / x)
            if not (currenty % 2):
                currentx = i % x
            else:
                currentx = x - (i % x) - 1
            self.locs.append((currentx,currenty))
    
    def iterate(self,num):
        # for each iteration, do one monte carlo swap
        for n in range(num):
            beginloc = randrange(2) # choose whether to start for beginning point or end point
            if beginloc:
                p = 0 # starter index
            else:
                p = len(self.locs)-1 # ending index
            pos = self.locs[p]
            neighbors = [(pos[0]-1,pos[1]),
            (pos[0]+1,pos[1]),
            (pos[0],pos[1]-1),
            (pos[0],pos[1]+1)] # find position of neighbors
            
            # remove already connected neighbor from list
            if beginloc: connecp = 1 # second index
            else: connecp = len(self.locs)-2 # one before ending index
            neighbors.remove(self.locs[connecp])
            
            # remove out of bounds neighbors
            
            neighbors = [n for n in neighbors if n[0] >= 0 and n[0] < self.dim[0] and n[1] >= 0 and n[1] < self.dim[1]]
            
            chooseneighbor = neighbors[randrange(len(neighbors))]
                    
            p2 = self.locs.index(chooseneighbor)

            #this is slower. an attempt was made.
            #locs_as_dict = dict(zip(self.locs,range(0,len(self.locs))))
            #p2 = locs_as_dict[chooseneighbor]
            
            # see the cited mansfield paper for info on this operation. involves reversing the list between
            # the two points
            if beginloc: # swap chosen point around beginning
                section = self.locs[0:p2:1]
                section.reverse()
                self.locs = section+self.locs[p2:]
            else: # swap chosen point around end
                section = self.locs[p2+1:]
                section.reverse()
                self.locs = self.locs[0:p2+1] + section
            
                
                
if __name__ == "__main__": # debugging demo - print starter 3x3 curve and curve after 10 iter
    curve = mansfieldcurve(9,9)
    print(curve.locs)
    curve.iterate(10)
    print(curve.locs)
    