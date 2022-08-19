import lorentz
import numpy as np


def test_default_parameters():
    assert (lorentz.default_parameters() == np.array([2.4, 21.0, 12.0])).all()


def test_default_states():
    assert (lorentz.default_states() == np.array([1.0, 2.0, 3.05])).all()


def test_solve():
    parameters = lorentz.default_parameters()
    states = lorentz.default_states()
    u = lorentz.solve(1, 0.01, states, parameters)
    assert u.shape == (100, 3)
