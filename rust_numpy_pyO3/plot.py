import lorentz
import matplotlib.pyplot as plt

parameters = lorentz.default_parameters()
states = lorentz.default_states()
u = lorentz.solve(100, 0.01, states, parameters)

fig = plt.figure()
ax = fig.add_subplot(projection="3d")
ax.plot(u[:, 0], u[:, 1], u[:, 2], lw=0.5)
ax.set_xlabel("X Axis")
ax.set_ylabel("Y Axis")
ax.set_zlabel("Z Axis")
ax.set_title("Lorenz Attractor")
plt.show()
