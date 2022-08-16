import matplotlib.pyplot as plt
import numpy as np

u = np.loadtxt("test.txt")

fig = plt.figure()
ax = fig.add_subplot(projection="3d")
ax.plot(u[:, 0], u[:, 1], u[:, 2], lw=0.5)
ax.set_xlabel("X Axis")
ax.set_ylabel("Y Axis")
ax.set_zlabel("Z Axis")
ax.set_title("Lorenz Attractor")
plt.show()
