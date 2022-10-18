from .diffeqr import solve_real
from typing import Callable


def complexify(v: list[complex]) -> list[float]:
    return list(map(lambda z: z.real, v)) + list(map(lambda z: z.imag, v))


def decomplexify(v: list[float]) -> list[complex]:
    if len(v) % 2 == 1:
        raise TypeError(
            "Complexified list must have an even number of elements")
    len2 = int(len(v)/2)
    res: list[complex] = [0.0, ]*(len2)
    for i in range(len2):
        if v[i+len2] == 0:
            res[i] = v[i]
        else:
            res[i] = v[i] + v[i+len2]*1j
    return res


def solve_ivp(fun: Callable[[float, list[complex]], list[complex]], t_span: tuple[float], y0: list[complex], **kwargs) -> list[complex]:
    """Solves a real initial value problem for a system of first order ODEs

    Args:
        fun (Callable[[float, list[complex]], list[complex]]): Function in the right-hand side of dy/dt = F(t, y)
        t_span (tuple): Start and end values for t
        y0 (list[complex]): Initial value for y at t0

    Returns:
        list[complex]: Solution to the IVP at tf
    """
    y0r = complexify(y0)

    def funr(t: float, y: list[float]) -> list[float]:
        return complexify(fun(t, decomplexify(y)))
    resr = solve_real(funr, t_span, y0r)
    return decomplexify(resr)
