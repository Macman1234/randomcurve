import imageio

time = 240

frames = []
for t in range(time):
    print(t)
    image = imageio.v2.imread(f'test{t}.png')
    frames.append(image)

imageio.mimsave('./example.gif', # output gif
                frames,          # array of input frames
                duration = 40)         # optional: frames per second